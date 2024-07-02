// // this file holds the main driver of our vault codebase
// use clap::{App, Arg};
// use rayon::prelude::*;
// use serde::{Deserialize, Serialize};
// use std::time::Instant;

// mod hash_generator;
// mod hash_sorter;
// mod print_records;
// mod store_hashes;

// #[allow(dead_code)]
// #[derive(Debug, Serialize, Deserialize)]

// struct Record {
//     nonce: [u8; 6], // nonce is always 6 bytes in size & unique; represented by an array of u8 6 elements
//     hash: [u8; 26],
// }

// fn main() {
//     // defines letters for arguments that the user can call from command line
//     let matches = App::new("Vault")
//         .version("2.0")
//         .about("Generates hashes for unique nonces using BLAKE3 hashing function. This vault also has the ability to store each record (nonce/hash pair) into a vector, sort them accordingly, and even look them up efficiently.")
//         .arg(
//             Arg::with_name("k-value")
//                 .short('k') // you can change this flag
//                 .long("k-value")
//                 .takes_value(true) // there must be a number inputted
//                 .help("Specify k value to compute 2^k nonces"),
//         )
//         .arg(
//             Arg::with_name("filename")
//                 .short('f')
//                 .long("filename")
//                 .takes_value(true)
//                 .help("Output file to store the generated hashes"),
//         )
//         .arg(
//             Arg::with_name("print")
//                 .short('p')
//                 .long("print")
//                 .takes_value(true)
//                 .help("Number of records to print"),
//         )
//         .arg(
//             Arg::with_name("sorting_on")
//                 .short('s')
//                 .long("sorting_on")
//                 .takes_value(true)
//                 .help("Turn sorting on/off"),
//         )
//         .arg(
//             Arg::with_name("threads")
//                 .short('t')
//                 .long("threads")
//                 .takes_value(true)
//                 .default_value("1")
//                 .help("Number of threads to use for hash generation"),
//         )
//         .get_matches();

//     let k = matches
//         .value_of("k-value")
//         .unwrap_or("0")
//         .parse::<u32>()
//         .expect("Please provide a valid integer for k");

//     let num_records = 2u64.pow(k);

//     let num_threads = matches
//         .value_of("threads")
//         .unwrap_or("4")
//         .parse::<usize>()
//         .expect("Please provide a valid number for threads");

//     let num_records_to_print = matches
//         .value_of("print")
//         .unwrap_or("0")
//         .parse::<u64>()
//         .expect("Please provide a valid number of records to print");

//     let output_file = matches.value_of("filename").unwrap_or("");

//     let sorting_on = matches
//         .value_of("sorting_on")
//         .unwrap_or("true")
//         .parse::<bool>()
//         .expect("Please provide a valid value for sorting_on (true/false)");

//     // libary to use multiple threads
//     rayon::ThreadPoolBuilder::new()
//         .num_threads(num_threads)
//         .build_global()
//         .unwrap();

//     let start_vault_timer: Instant = Instant::now();
//     let start_hash_gen_timer: Instant = Instant::now();

//     // generate hashes in parallel (if using multiple threads)
//     let mut hashes: Vec<Record> = (0..num_records)
//         .into_par_iter()
//         .map(hash_generator::generate_hash) // Now directly maps each nonce to a Record
//         .collect();

//     let hash_gen_duration = start_hash_gen_timer.elapsed();
//     println!(
//         "Generating {} hashes took {:?}",
//         num_records, hash_gen_duration
//     );

//     let sorting_timer: Instant = Instant::now();

//     if sorting_on {
//         hash_sorter::sort_hashes(&mut hashes);
//     }

//     let start_store_output_timer: Instant = Instant::now();

//     let sorting_finished = sorting_timer.elapsed();
//     println!("Sorting them sequentially takes {:?}", sorting_finished);

//     if output_file != "" {
//         match store_hashes::store_hashes(&hashes, output_file, &num_threads) {
//             Ok(_) => println!("Hashes successfully written to {}", output_file),
//             Err(e) => eprintln!("Error writing hashes to file: {}", e),
//         }
//     }

//     let store_output_duration: std::time::Duration = start_store_output_timer.elapsed();
//     println!("Writing hashes to disk took {:?}", store_output_duration);

//     let duration = start_vault_timer.elapsed();
//     print!("Generated");
//     if sorting_on {
//         print!(", sorted");
//     }
//     if !output_file.is_empty() {
//         print!(", stored");
//     }
//     println!(" {} records in {:?}", num_records, duration);

//     if num_records_to_print != 0 {
//         match print_records::print_records(output_file, num_records_to_print) {
//             Ok(_) => println!("Hashes successfully deserialized from {}", output_file),
//             Err(e) => eprintln!("Error deserializing hashes: {}", e),
//         }
//     }
// }

// this method uses a DashMap to store prefixes
use clap::{App, Arg};
use dashmap::DashMap;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Instant;

mod hash_generator;
mod print_records;
mod store_hashes;

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    nonce: [u8; 6], // nonce is always 6 bytes in size & unique; represented by an array of u8 6 elements
    hash: [u8; 26],
}

fn main() {
    let matches = App::new("Vault")
        .version("2.0")
        .about("Generates hashes for unique nonces using BLAKE3 hashing function.")
        .arg(
            Arg::with_name("k-value")
                .short('k')
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

    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();

    let output_file = matches.value_of("filename").unwrap_or("");

    let generation_start = Instant::now();

    (0..num_records)
        .into_par_iter()
        .map(|nonce| hash_generator::generate_hash(nonce, prefix_length))
        .for_each(|(prefix, record)| {
            map.entry(prefix).or_insert_with(Vec::new).push(record);
        });

    let generation_duration = generation_start.elapsed();
    println!("Hash generation took {:?}", generation_duration);

    let storage_start = Instant::now();

    // if !output_file.is_empty() {
    //     if let Err(e) = store_hashes::store_hashes_dashmap(&map, output_file) {
    //         eprintln!("Error writing hashes to file: {}", e);
    //     }
    // }
    if !output_file.is_empty() {
        let _ = store_hashes::store_hashes_dashmap(&map, output_file);
    }

    let storage_duration = storage_start.elapsed();
    println!("Writing hashes to disk took {:?}", storage_duration);

    let total_duration = generation_duration + storage_duration;

    // let num_buckets: u128 = 2u128.pow((prefix_length * 8) as u32);

    // snippet to check the contents of the map
    let num_keys = map.len();
    println!("Total number of unique prefix buckets: {}", num_keys);

    let average_records_per_key =
        map.iter().map(|entry| entry.value().len()).sum::<usize>() as f64 / num_keys as f64;
    println!(
        "Average number of records per prefix bucket: {:.2}",
        average_records_per_key
    );

    let total_records = map.iter().map(|entry| entry.value().len()).sum::<usize>();
    println!("Total number of records stored: {}", total_records);

    if total_records == num_records as usize {
        println!("The total number of records is correct!.");
    } else {
        println!(
            "Mismatch: the total number of records does not match 2^25. Found {}",
            total_records
        );
    }

    // If you want to see details of each bucket, uncomment the following lines
    // map.iter().for_each(|entry| {
    //     println!("Prefix {:?} has {} records", *entry.key(), entry.value().len());
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
