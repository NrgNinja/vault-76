// this file holds the main driver of our vault codebase
use clap::{App, Arg};
use rand::random;
use rayon::iter::ParallelIterator;
use rayon::prelude::*;
use rayon::slice::ParallelSlice;
use serde::{Deserialize, Serialize};
use std::time::Instant;

mod hash_generator;
mod hash_sorter;
mod lookup;
mod print_records;
mod store_hashes;

pub const NONCE_SIZE: usize = 6;
pub const HASH_SIZE: usize = 26;
pub const RECORD_SIZE: usize = 32;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Record {
    nonce: [u8; NONCE_SIZE], // nonce is always 6 bytes in size & unique; represented by an array of u8 6 elements
    hash: [u8; HASH_SIZE],
}

fn main() {
    // defines letters for arguments that the user can call from command line
    let matches = App::new("Vault")
        .version("2.0")
        .about("Generates hashes for unique nonces using BLAKE3 hashing function. This vault also has the ability to store each record (nonce/hash pair) into a vector, sort them accordingly, and even look them up efficiently.")
        .arg(
            Arg::with_name("k-value")
                .short('k') // you can change this flag
                .long("k-value")
                .takes_value(true) // there must be a number inputted
                .help("Specify k value to compute 2^k nonces"),
        )
        .arg(
            Arg::with_name("writing_on")
                .short('w')
                .long("writing_on")
                .takes_value(true)
                .help("Output file to store the generated hashes"),
        )
        .arg(
            Arg::with_name("print")
                .short('p')
                .long("print")
                .takes_value(true)
                .help("Number of records to print"),
        )
        .arg(Arg::with_name("print_index").short('i').long("print_index").takes_value(true).help("Print index file"))
        .arg(
            Arg::with_name("sorting_on")
                .short('s')
                .long("sorting_on")
                .takes_value(true)
                .help("Turn sorting on/off"),
        )
        .arg(
            Arg::with_name("threads")
                .short('t')
                .long("threads")
                .takes_value(true)
                .default_value("1") 
                .help("Number of threads to use for hash generation"),
        )
        .arg(
            Arg::with_name("target_hash")
                .short('x')
                .long("target_hash")
                .takes_value(true)
                .help("String hash to lookup from the data"),
        )
        .get_matches();

    let k = matches
        .value_of("k-value")
        .unwrap_or("0")
        .parse::<u32>()
        .expect("Please provide a valid integer for k");

    let num_records = 2u64.pow(k);

    let num_threads = matches
        .value_of("threads")
        .unwrap_or("1")
        .parse::<usize>()
        .expect("Please provide a valid number for threads");

    let num_records_to_print = matches
        .value_of("print")
        .unwrap_or("0")
        .parse::<u64>()
        .expect("Please provide a valid number of records to print");

    let print_index = matches
        .value_of("print_index")
        .unwrap_or("false")
        .parse::<bool>()
        .expect("Please provide a valid number of records to print");

    let writing_on = matches
        .value_of("writing_on")
        .unwrap_or("true")
        .parse::<bool>()
        .expect("Please provide a valid value for writing to disk on/off (true/false)");

    let sorting_on = matches
        .value_of("sorting_on")
        .unwrap_or("true")
        .parse::<bool>()
        .expect("Please provide a valid value for sorting_on (true/false)");

    let target_hash = matches.value_of("target_hash").unwrap_or("0");

    let directory = "output";

    // libary to use multiple threads
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();

    // let start_vault_timer: Instant = Instant::now();

    if k != 0 {
        // println!("Populating the vault...");
        let start_hash_gen_timer: Instant = Instant::now();

        let mut hashes: Vec<Record> = (0..num_records)
            .into_par_iter()
            .map(|_| {
                let nonce: u64 = random();
                hash_generator::generate_hash(nonce)
            }) // Now directly maps each nonce to a Record
            .collect();

        let chunk_size: usize = hashes.len() / num_threads;

        let hash_gen_duration = start_hash_gen_timer.elapsed();
        // println!(
        //     "Generating {} hashes took {:?}",
        //     num_records, hash_gen_duration
        // );

        let mut hash_sort_duration = std::time::Duration::new(0, 0);

        // Calls a function that sorts hashes in memory (hash_sorter.rs)
        let start_hash_sort_timer: Instant = Instant::now();
        if sorting_on {
            hash_sorter::sort_hashes(&mut hashes);
            // println!("Sorting hashes took {:?}", hash_sort_duration);
            hash_sort_duration = start_hash_sort_timer.elapsed();
        }

        let mut store_output_duration: std::time::Duration = std::time::Duration::new(0, 0);

        // Calls store_hashes function to serialize generated hashes into binary and store them on disk -- sometimes takes 4-8sec
        if writing_on {
            let start_store_output_timer: Instant = Instant::now();
            let index_file_path = "output/file_index.bin";

            let results = hashes
                .par_chunks(chunk_size)
                .map(|chunk| {
                    let first_hash = hex::encode(chunk.first().unwrap().hash);
                    let last_hash = hex::encode(chunk.last().unwrap().hash);
                    let chunk_filename = format!("{}-{}.bin", first_hash, last_hash);

                    store_hashes::store_hashes_chunk(chunk, &chunk_filename)
                        .expect("Failed to store hashes");

                    (first_hash, last_hash, chunk_filename)
                })
                .collect();

            store_hashes::create_index_file(index_file_path, results)
                .expect("Failed to create index file");

            store_output_duration = start_store_output_timer.elapsed();
            // println!("Writing hashes to disk took {:?}", store_output_duration);

            if print_index {
                print_records::print_index_file(index_file_path)
                    .expect("Failed to print index file");
            }
        }

        // let duration = start_vault_timer.elapsed();
        // print!("Generated");
        // if sorting_on {
        //     print!(", sorted");
        // }
        // if writing_on {
        //     print!(", stored");
        // }
        // println!(" {} records in {:?}", num_records, duration);
        println!(
            "{:?},{:?},{:?}",
            hash_gen_duration, hash_sort_duration, store_output_duration
        )
    }

    if num_records_to_print != 0 {
        match print_records::print_records(directory, num_records_to_print) {
            Ok(_) => println!("Hashes successfully deserialized from {}", directory),
            Err(e) => eprintln!("Error deserializing hashes: {}", e),
        }
    }

    if target_hash != "0" {
        let start_lookup_timer = Instant::now();

        let mut is_exist: bool = false;

        // Single-threaded
        match lookup::lookup_hash_in_file(directory, &target_hash) {
            Ok(Some(_record)) => {
                // let hash = print_records::hash_to_string(&record.hash);
                // let nonce = print_records::nonce_to_decimal(&record.nonce);

                is_exist = true;
            }
            Ok(None) => is_exist = false,
            Err(e) => eprintln!("Error occurred: {}", e),
        }

        let lookup_duration = start_lookup_timer.elapsed();
        println!("{},{:?},{}", target_hash, lookup_duration, is_exist);
    }
}
