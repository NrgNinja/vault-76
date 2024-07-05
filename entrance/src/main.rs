// this file holds the main driver of our vault codebase
use clap::{App, Arg};
use dashmap::DashMap;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Instant;

mod hash_generator;
mod print_records;
mod store_hashes;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]

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
                .default_value("2") // Set a default value or make it required
                .help("Specify the prefix length in bytes to categorize the hashes"),
        )
        .get_matches();

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

    // --------------------------------------------------------------------------------------------
    // use blake3::{Hasher, Hash};

    // let num_hashes_only = 33554432; // Replace with the number of hashes you want to generate
    // let start_only = Instant::now();

    // (0..num_hashes_only).into_par_iter().for_each(|_| {
    //     let mut hasher_only = Hasher::new();
    //     hasher_only.update(b"example data");
    //     let _hash_only = hasher_only.finalize();
    // });

    // let duration_only = start_only.elapsed();
    // println!(
    //     "Time taken to generate {} hashes only: {:?}",
    //     num_hashes_only, duration_only
    // );

    // GENERATE HASHES ONLY ^----------------------------------------------------------------------

    // --------------------------------------------------------------------------------------------

    // let start_and = Instant::now();

    // let _hashes_and: Vec<Hash> = (0..num_hashes_only)
    //     .into_par_iter()
    //     .map(|_| {
    //         let mut hasher_and = Hasher::new();
    //         hasher_and.update(b"example data");
    //         hasher_and.finalize()
    //     })
    //     .collect();

    // let duration_and = start_and.elapsed();
    // println!(
    //     "Time taken to generate and store {} hashes into a vector: {:?}",
    //     num_hashes_only, duration_and
    // );

    // GENERATE AND STORE HASHES INTO VECTOR ^-----------------------------------------------------

    let generation_start = Instant::now();

    // generate hashes in parallel (if using multiple threads)
    (0..num_records)
        .into_par_iter()
        .map(|nonce| hash_generator::generate_hash(nonce, prefix_length))
        .for_each(|(prefix, record)| {
            map.entry(prefix).or_insert_with(Vec::new).push(record);
        });

    let generation_duration = generation_start.elapsed();
    println!(
        "Hash generation & storing into DashMap took {:?}",
        generation_duration
    );

    let storage_start = Instant::now();

    if !output_file.is_empty() {
        let _ = store_hashes::store_hashes_dashmap(&map, output_file);
    }

    let storage_duration = storage_start.elapsed();
    println!("Writing hashes to disk took about {:?}", storage_duration);

    let total_duration = generation_duration + storage_duration;

    // snippet to check the contents of the map
    let num_keys = map.len();
    // println!("Total number of unique prefix buckets: {}", num_keys);

    // let average_records_per_key =
    //     map.iter().map(|entry| entry.value().len()).sum::<usize>() as f64 / num_keys as f64;
    // println!(
    //     "Average number of records per prefix bucket: {:.2}",
    //     average_records_per_key
    // );

    let total_records = map.iter().map(|entry| entry.value().len()).sum::<usize>();
    // println!("Total number of records stored: {}", total_records);

    // if you want to see details of each bucket, uncomment the following lines
    // map.iter().for_each(|entry| {
    //     println!("Prefix bucket {} has {} records", *entry.key(), entry.value().len());
    // });

    println!(
        "Time taken for {} parallel insertions into {} buckets using {} threads: {:?}",
        total_records, num_keys, num_threads, total_duration
    );

    if let Some(num_records_to_print) = matches
        .value_of("print")
        .map(|v| v.parse::<usize>().unwrap())
    {
        print_records::print_records_dashmap(&map, num_records_to_print);
    }
}
