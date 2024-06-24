// this file will hold the main driver of our vault codebase
use clap::{App, Arg};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Instant;

mod hash_generator;
mod hash_sorter;
mod print_records;
mod store_hashes;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]

struct Record {
    nonce: [u8; 6], // nonce is always 6 bytes in size and unique. Represented by an array of u8 6 elements
    hash: [u8; 26],
}

fn main() {
    // defines letters for arguments that the user can call from Command Line
    let matches = App::new("Vault")
        .version("2.0")
        .about("Generates hashes for unique nonces using BLAKE3 hashing function. This vault also has the ability to store each record (nonce/hash pair) into a vector, sort them accordingly, and even look them up efficiently.")
        .arg(
            Arg::with_name("nonces")
                .short('n') // you can change this flag
                .long("nonces")
                .takes_value(true) // there must be a number inputted
                .help("Number of nonces to generate hashes for"),
        )
        .arg(
            Arg::with_name("filename")
                .short('f') // you can change this flag
                .long("filename")
                .takes_value(true) // there must be a filename inputted
                .help("Output file to store the generated hashes"),
        )
        .arg(
            Arg::with_name("print")
                .short('p') // you can change this flag to whatever you want filename to represent
                .long("print")
                .takes_value(true) // there must be a filename inputted
                .help("Number of records to print"),
        )
        .arg(
            Arg::with_name("sorting_on")
                .short('s') // you can change this flag to whatever you want filename to represent
                .long("sorting_on")
                .takes_value(true) // there must be a filename inputted
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

    // Defines a variable to store the number of records to generate
    let num_records = matches
        .value_of("nonces")
        .unwrap_or("10") // default value if none specified
        .parse::<u64>() // parse it into 64 bit unsigned int
        .expect("Please provide a valid number for nonces");

    let num_threads = matches
        .value_of("threads")
        .unwrap_or("4")
        .parse::<usize>()
        .expect("Please provide a valid number for threads");

    // Defines a variable to store the number of hashes to print
    let num_records_to_print = matches
        .value_of("print")
        .unwrap_or("0")
        .parse::<u64>()
        .expect("Please provide a valid number of records to print");

    // output file to store binary format of hashes
    let output_file = matches.value_of("filename").unwrap_or("");

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

    let start_vault_timer: Instant = Instant::now();

    // generate hashes in parallel
    let start_hash_gen_timer: Instant = Instant::now();

    let mut hashes: Vec<Record> = (0..num_records)
        .into_par_iter()
        .map(hash_generator::generate_hash) // Now directly maps each nonce to a Record
        .collect();

    let hash_gen_duration = start_hash_gen_timer.elapsed();
    println!("Generating hashes took {:?}", hash_gen_duration);

    // Calls a function that sorts hashes in memory (hash_sorter.rs)
    if sorting_on {
        hash_sorter::sort_hashes(&mut hashes);
    }

    let start_store_output_timer: Instant = Instant::now();

    // Calls store_hashes function to serialize generated hashes into binary and store them on disk
    if output_file != "" {
        match store_hashes::store_hashes(&hashes, output_file) {
            Ok(_) => println!("Hashes successfully written to {}", output_file),
            Err(e) => eprintln!("Error writing hashes to file: {}", e),
        }
    }

    let store_output_duration: std::time::Duration = start_store_output_timer.elapsed();
    println!("Writing hashes to disk took {:?}", store_output_duration);

    // Final print statement with total duration of the program
    let duration = start_vault_timer.elapsed();
    print!("Generated");
    if sorting_on {
        print!(", sorted");
    }
    if !output_file.is_empty() {
        print!(", stored");
    }
    println!(" {} records in {:?}", num_records, duration);

    // Calls print_records function to deserialize and print all of the records into command prompt
    if num_records_to_print != 0 {
        match print_records::print_records(output_file, num_records_to_print) {
            Ok(_) => println!("Hashes successfully deserialized from {}", output_file),
            Err(e) => eprintln!("Error deserializing hashes: {}", e),
        }
    }
}
