// this file will hold the main driver of our vault codebase
use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use rayon::prelude::*;
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
    // Defines letters for arguments that the user can call from Command Line
    let matches = App::new("Vault")
        .version("1.0")
        .about("Generates hashes for specified nonces using BLAKE3")
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
        .parse::<usize>() // parse it into 64 bit unsigned int
        .expect("Please provide a valid number for nonces");

    let num_threads = matches
        .value_of("threads")
        .unwrap_or("4")
        .parse::<usize>()
        .expect("Please provide a valid number for threads");

    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();

    // // output file to store binary format of hashes
    // let output_file = matches.value_of("filename").unwrap_or("output.bin");

    // Defines a variable to check if the sorting mechanism should happen or not
    let sorting_on = matches
        .value_of("sorting_on")
        .unwrap_or("true")
        .parse::<bool>()
        .expect("Please provide a valid value for sorting_on (true/false)");

    // Start the timer
    let start = Instant::now();

    // generate hashes in parallel
    let _hashes: Vec<_> = (0..num_nonces)
        .into_par_iter()
        .map(|nonce| hash_generator::generate_hash(nonce as u64)) // Cast usize to u64 here
        .collect();

    // TODO: RENATO FIND A WAY TO MERGE THESE TWO!!!!!!!!!!!!!! ^
    // Calls a function that generates hashes and saves them into Vec<Record>
    let mut hashes = hash_generator::generate_hashes(num_records);

    // // calls hash_sorter.rs
    // // TODO: fix this sorter, make it shorter OR use built in vector sort
    // hash_sorter::sort_hashes(&mut hashes);


    // Calls a function that sorts hashes in memory
    if sorting_on {
        hash_sorter::sort_hashes(&mut hashes);
    }

    // Calls store_hashes function to serialize generated hashes into binary and store them on disk
    match store_file::store_hashes(&hashes, output_file) {
        Ok(_) => println!("Hashes successfully written to {}", output_file),
        Err(e) => eprintln!("Error writing hashes to file: {}", e),
    }

    // // calls store_file.rs
    // match store_file::store_hashes(&hashes, output_file) {
    //     Ok(_) => println!("Hashes successfully written to {}", output_file),
    //     Err(e) => eprintln!("Error writing hashes to file: {}", e),
    // }
    
    // // // for viewing the generated hashes
    // // for (nonce, hash) in hashes {
    // //     println!("Nonce: {} | {}", nonce, hash);
    // // }

    // Calls print_records function to deserialize and print all of the records into command prompt
    match print_records::print_records(output_file, num_records_to_print) {
        Ok(_) => println!("Hashes successfully deserialized from {}", output_file),
        Err(e) => eprintln!("Error deserializing hashes: {}", e),
    }

    let duration = start.elapsed();
    println!("Generated {} in {:?}", num_records, duration);
}
