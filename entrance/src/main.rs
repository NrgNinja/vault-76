// this file is the main driver of the vault 76 codebase

// different files of the vault 76 toolkit:
// generation of hashes    --> hash-generator.rs
// organize the hashes     --> hash-organizer.rs
// sort the hashes         --> hash-sorter.rs
// store the hashes        --> store-file.rs

// ignore unused code
#[allow(dead_code)]
use clap::{App, Arg};
use rayon::prelude::*;
use std::time::Instant;

mod hash_generator;
mod hash_sorter;
mod store_file;

fn main() {
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
            Arg::with_name("threads")
                .short('t') // cmd line flag
                .long("threads")
                .takes_value(true)
                .default_value("1") // there must be threads specified
                .help("Number of threads to use for hash generation"),
        )
        .get_matches();

    let num_nonces = matches
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

    // start the timer
    let start = Instant::now();

    // generate hashes in parallel
    let _hashes: Vec<_> = (0..num_nonces)
        .into_par_iter()
        .map(|nonce| hash_generator::generate_hash(nonce as u64)) // Cast usize to u64 here
        .collect();

    // // calls hash_sorter.rs
    // // TODO: fix this sorter, make it shorter OR use built in vector sort
    // hash_sorter::sort_hashes(&mut hashes);

    // // // for viewing the generated hashes
    // // for (nonce, hash) in hashes {
    // //     println!("Nonce: {} | {}", nonce, hash);
    // // }

    // // calls store_file.rs
    // match store_file::store_hashes(&hashes, output_file) {
    //     Ok(_) => println!("Hashes successfully written to {}", output_file),
    //     Err(e) => eprintln!("Error writing hashes to file: {}", e),
    // }

    // end the timer
    let duration = start.elapsed();
    println!("Generated {} in {:?}", num_nonces, duration);
}
