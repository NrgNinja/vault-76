// this file holds the main driver of our vault codebase
use clap::{App, Arg};
use dashmap::DashMap;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Instant;

mod hash_generator;
mod lookup;
mod print_records;
mod store_hashes;

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize)]

struct Record {
    nonce: [u8; 6], // nonce is always 6 bytes in size & unique; represented by an array of u8 6 elements
    hash: [u8; 26],
}

// this method uses a DashMap to store prefixes
fn main() {
    // defines letters for arguments that the user can call from command line
    let matches = App::new("Vault")
        .version("2.0")
        .about("Generates hashes for unique nonces using BLAKE3 hashing function.")
        .arg(
            Arg::with_name("k-value")
                .short('k') // you can change these flags
                .long("k-value")
                .takes_value(true)
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
            Arg::with_name("threads")
                .short('t')
                .long("threads")
                .takes_value(true)
                .default_value("1")
                .help("Number of threads to use for hash generation"),
        )
        .arg(
            Arg::with_name("sorting_on")
                .short('s')
                .long("sorting_on")
                .takes_value(true)
                .help("Turn sorting on/off"),
        )
        .arg(
            Arg::with_name("prefix-length")
                .short('x')
                .long("prefix-length")
                .takes_value(true)
                .default_value("2") // set a default value or make it required
                .help("Specify the prefix length in bytes to categorize the hashes"),
        )
        .arg(
            Arg::with_name("lookup")
                .short('l')
                .long("lookup")
                .takes_value(true)
                .help("Lookup a record by nonce, hash, or prefix"),
        )
        .get_matches();

    // determine if lookup is specified, otherwise continue normal vault operations
    if let Some(lookup_value) = matches.value_of("lookup") {
        if let Err(e) =
            lookup::lookup_by_prefix(matches.value_of("filename").unwrap_or(""), lookup_value)
        {
            eprintln!("Error during lookup: {}", e);
        }
        return;
    }

    let k = matches
        .value_of("k-value")
        .unwrap_or("0")
        .parse::<u32>()
        .unwrap();

    let num_records = 2u64.pow(k);

    let num_threads = matches
        .value_of("threads")
        .unwrap_or("4")
        .parse::<usize>()
        .unwrap();

    let prefix_length = matches
        .value_of("prefix-length")
        .unwrap_or("2")
        .parse::<usize>()
        .expect("Please provide a valid integer for prefix length");

    let map = DashMap::new();

    // libary to use multiple threads
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();

    let output_file = matches.value_of("filename").unwrap_or("");

    let generation_start = Instant::now();

    // generate hashes in parallel and store them into a DashMap data structure
    (0..num_records)
        .into_par_iter()
        .map(|nonce| hash_generator::generate_hash(nonce, prefix_length))
        .for_each(|(prefix, record)| {
            map.entry(prefix).or_insert_with(Vec::new).push(record);
        });

    let generation_duration = generation_start.elapsed();
    // println!(
    //     "Hash generation & storing into DashMap took {:?}",
    //     generation_duration
    // );

    let storage_start = Instant::now();

    // if an output file is specified by the command line, it will write to that file
    if !output_file.is_empty() {
        let _ = store_hashes::store_hashes_optimized(&map, output_file);
    }

    let storage_duration = storage_start.elapsed();
    // println!("Writing hashes to disk took about {:?}", storage_duration);

    // let total_duration = generation_duration + storage_duration;

    // check the contents of the map
    // let num_keys = map.len();
    // let total_records = map.iter().map(|entry| entry.value().len()).sum::<usize>();

    // if you want to see details of each prefix bucket, uncomment the following lines
    // let mut keys_with_counts: Vec<(u64, usize)> = map
    //     .iter()
    //     .map(|entry| (*entry.key(), entry.value().len()))
    //     .collect();

    // keys_with_counts.sort_by_key(|k| k.0);

    // for (key, count) in keys_with_counts {
    //     let prefix_hex = format!("{:x}", key); // convert numeric prefix to hex
    //     println!("Prefix bucket {} has {} records", prefix_hex, count);
    // }

    // println!(
    //     "Time taken for {} parallel insertions into {} buckets using {} threads: {:?}",
    //     total_records, num_keys, num_threads, total_duration
    // );

    println!(
        "{:?},{:?}",
        generation_duration, storage_duration
    );

    // if you specify a number of records to print to the screen; for debugging purposes
    if let Some(num_records_to_print) = matches
        .value_of("print")
        .map(|v| v.parse::<usize>().unwrap())
    {
        print_records::print_records_from_file(num_records_to_print as u64).unwrap();
    }
}
