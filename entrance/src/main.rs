// this file is the main driver of the vault codebase
use clap::{App, Arg};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Instant;

mod hash_generator;
mod hash_sorter;
mod print_records;
mod store_file;

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    nonce: u64, // nonce is always 6 bytes in size and unique. Represented by an array of u8 6 elements
    hash: String,
}

fn main() {
    // defines letters for arguments that the user can call from Command Line
    let matches = App::new("Vault-76")
        .version("2.0")
        .about("Generates hashes for unique nonces using BLAKE3 hashing function. This vault also has the ability to store each record (nonce/hash pair) into a vector, sort them accordingly, and even look them up efficiently.")
        .arg(
            Arg::with_name("nonces")
                .short('n') // cmd line flag
                .long("nonces")
                .takes_value(true) // there must be a number inputted
                .help("Number of nonces to generate hashes for"),
        )
        .arg(
            Arg::with_name("filename")
                .short('f') // cmd line flag
                .long("filename")
                .takes_value(true) // there must be a filename inputted
                .help("Output file to store the generated hashes"),
        )
        .arg(
            Arg::with_name("print")
                .short('p') // cmd line flag
                .long("print")
                .takes_value(true) // there must be a number inputted
                .help("Number of records to print"),
        )
        .arg(
            Arg::with_name("sorting_on")
                .short('s') // cmd line flag
                .long("sorting_on")
                .takes_value(true) // there must be a boolean inputted
                .help("Turn sorting on/off"),
        )
        .arg(
            Arg::with_name("threads")
                .short('t') // cmd line flag
                .long("threads")
                .takes_value(true)
                .default_value("1") // there must be threads specified
                .help("Number of threads to use for hash generation"),
        )
        .get_matches();

    // variable to store the number of records to generate
    let num_records = matches
        .value_of("nonces")
        .unwrap_or("10") // default value if none specified
        .parse::<usize>() // parse it into 64 bit unsigned int
        .expect("Please provide a valid number for nonces");

    // variable to store the number of threads to use
    let num_threads = matches
        .value_of("threads")
        .unwrap_or("4")
        .parse::<usize>()
        .expect("Please provide a valid number for threads");

    // variable to store the number of records to print
    let num_records_to_print = matches
        .value_of("print")
        .unwrap_or("")
        .parse::<u64>()
        .expect("Please provide a valid number of records to print");

    // output file to store binary format of hashes
    let output_file = matches.value_of("filename").unwrap_or("output.bin");

    // Defines a variable to check if the sorting mechanism should happen or not
    let sorting_on = matches
        .value_of("sorting_on")
        .unwrap_or("true")
        .parse::<bool>()
        .expect("Please provide a valid value for sorting_on (true/false)");

    // libary to use multiple threads
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();

    let start = Instant::now();

    // generate hashes in parallel
    let mut hashes: Vec<Record> = (0..num_records)
        .into_par_iter()
        .flat_map(|nonce| hash_generator::generate_hashes(nonce)) // Cast usize to u64 here
        .collect();

    // Calls a function that sorts hashes in memory (hash_sorter.rs)
    if sorting_on {
        hash_sorter::sort_hashes(&mut hashes);
    }

    // Calls store_hashes function to serialize generated hashes into binary and store them on disk
    match store_file::store_hashes(&hashes, output_file) {
        Ok(_) => println!("Hashes successfully written to {}", output_file),
        Err(e) => eprintln!("Error writing hashes to file: {}", e),
    }

    // Calls print_records function to deserialize and print all of the records into command prompt
    if num_records_to_print != 0 {
        match print_records::print_records(output_file, num_records_to_print) {
            Ok(_) => println!("Hashes successfully deserialized from {}", output_file),
            Err(e) => eprintln!("Error deserializing hashes: {}", e),
        }
    }

    let duration = start.elapsed();
    println!("Generated {} in {:?}", num_records, duration);
}
