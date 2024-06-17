// this file is the main driver of the vault 76 codebase

// different files of the vault 76 toolkit:
// generation of hashes    --> hash-generator.rs
// organize the hashes     --> hash-organizer.rs
// sort the hashes         --> hash-sorter.rs
// store the hashes        --> store-file.rs

// ignore unused code
#[allow(dead_code)]
use clap::{App, Arg};
use std::time::Instant;

mod hash_sorter;
mod store_file;
mod hash_generator;

fn main() {
    let matches = App::new("Vault")
        .version("1.0")
        .about("Generates hashes for specified nonces using BLAKE3")
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
        .get_matches();

    let num_nonces = matches
        .value_of("nonces")
        .unwrap_or("10") // default value if none specified
        .parse::<u64>() // parse it into 64 bit unsigned int
        .expect("Please provide a valid number for nonces");

    let output_file = matches.value_of("filename").unwrap_or("main.bin");

    // start the timer
    let start = Instant::now();

    // calls hash_generator.rs
    let mut hashes = hash_generator::generate_hashes(num_nonces);

    // calls hash_sorter.rs
    hash_sorter::sort_hashes(&mut hashes);

    // // for viewing the generated hashes
    // for (nonce, hash) in hashes {
    //     println!("Nonce: {} | {}", nonce, hash);
    // }

    // calls store_file.rs
    match store_file::store_hashes(&hashes, output_file) {
        Ok(_) => println!("Hashes successfully written to {}", output_file),
        Err(e) => eprintln!("Error writing hashes to file: {}", e),
    }

    // end the timer
    let duration = start.elapsed();
    println!("Generated {} in {:?}", num_nonces, duration);
}
