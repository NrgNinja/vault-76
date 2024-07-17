// this file holds the main driver of our vault codebase
use clap::{App, Arg};
use dashmap::DashMap;
use hash_generator::generate_hash;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::RwLock;
use std::time::Instant;
use store_hashes::flush_to_disk;

mod hash_generator;
mod print_records;
mod store_hashes;

const RECORD_SIZE: usize = 32; // 6 bytes for nonce + 26 bytes for hash
const HASH_SIZE: usize = 26;
const NONCE_SIZE: usize = 6;

#[derive(Debug, Serialize, Deserialize)]
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
            Arg::with_name("filename")
                .short('f')
                .long("filename")
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
        .arg(Arg::with_name("memory_limit")
        .short('m')
        .long("memory_limit")
        .takes_value(true)
        .help("Limit memory"),)
        .arg(Arg::with_name("prefix_length").short('x').long("prefix").takes_value(true).help("Specify the prefix length to extract from the hash"))
        .get_matches();

    let k = matches
        .value_of("k-value")
        .unwrap_or("0")
        .parse::<u32>()
        .expect("Please provide a valid integer for k");

    let num_records = 2u64.pow(k);

    let num_threads = matches
        .value_of("threads")
        .unwrap_or("4")
        .parse::<usize>()
        .expect("Please provide a valid number for threads");

    let num_records_to_print = matches
        .value_of("print")
        .unwrap_or("0")
        .parse::<u64>()
        .expect("Please provide a valid number of records to print");

    let output_file = matches.value_of("filename").unwrap_or("");

    let memory_limit = matches
        .value_of("memory_limit")
        .unwrap_or("2147483648")
        .parse::<usize>()
        .expect("Please provide a valid number for memory_limit");

    let prefix_length = matches
        .value_of("prefix_length")
        .unwrap_or("2")
        .parse::<usize>()
        .expect("Please provide a valid number for prefix_length");

    // libary to use multiple threads
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();

    let start_vault_timer = Instant::now();

    let total_memory: usize = (num_records * RECORD_SIZE as u64).try_into().unwrap(); // in bytes
    let dashmap_capacity = memory_limit / RECORD_SIZE;

    let map: DashMap<usize, Vec<Record>> = DashMap::with_capacity(dashmap_capacity);
    let thread_memory_limit; // in bytes

    if total_memory < memory_limit {
        thread_memory_limit = total_memory / num_threads;
    } else {
        thread_memory_limit = memory_limit / num_threads; // in bytes
    }
    let mut total_generated = 0;

    let num_buckets = 1 << (prefix_length * 8); // Calculate number of buckets
    let records_in_bucket = num_records / num_buckets as u64;

    let offsets_vector: RwLock<Vec<usize>> = RwLock::new(vec![0; num_buckets]);

    while total_generated < total_memory {
        (0..num_threads).into_par_iter().for_each(|thread_index| {
            let mut local_size = 0;
            let mut nonce: u64 = (thread_index * (thread_memory_limit / RECORD_SIZE))
                .try_into()
                .unwrap();

            while local_size < thread_memory_limit {
                let (prefix, record) = generate_hash(nonce, prefix_length);

                nonce += 1;

                let mut records = map.entry(prefix as usize).or_insert_with(|| Vec::new());

                if records.len() >= records_in_bucket as usize {
                    continue;
                }
                records.push(record);
                local_size += RECORD_SIZE;

                // println!(
                //     "Records generated for prefix {}: {:?}",
                //     prefix,
                //     records.len()
                // );
            }
        });

        flush_to_disk(&map, &output_file, &offsets_vector).expect("Error flushing to disk");
        total_generated += thread_memory_limit * num_threads;
        map.clear();
    }

    // println!("Offsets vector: {:?}", offsets_vector);

    // Assuming record generation and processing - og version
    // (0..num_records)
    //     .into_par_iter()
    //     .map(|nonce| hash_generator::generate_hash(nonce, prefix_length))
    //     .for_each(|(prefix, record)| {
    //         let mut records = map.entry(prefix).or_insert_with(Vec::new);
    //         if records.len() >= records_per_thread {
    //             store_hashes::flush_to_disk(&records, &output_file);
    //             records.clear();
    //         }
    //         records.push(record);
    //     });

    // if an output file is specified by the command line, it will write to that file
    // if !output_file.is_empty() {
    //     let _ = store_hashes::store_hashes_optimized(
    //         &map,
    //         &output_file,
    //         memory_limit_bytes,
    //         RECORD_SIZE,
    //     );
    // }

    let duration = start_vault_timer.elapsed();
    print!("Generated");
    if !output_file.is_empty() {
        print!(", stored");
    }
    println!(" {} records in {:?}", num_records, duration);

    let offsets_vector_read = offsets_vector.read().unwrap(); // Use .unwrap() for simplicity in examples; handle errors as appropriate in production code
    println!(
        "Length of the offsets vector: {}",
        offsets_vector_read.len()
    );

    if num_records_to_print != 0 {
        match print_records::print_records_from_file(num_records_to_print) {
            Ok(_) => println!("Hashes successfully deserialized from {}", output_file),
            Err(e) => eprintln!("Error deserializing hashes: {}", e),
        }
    }
}
