// this file will hold the main driver of our vault codebase

// we can separate different components of the toolkit into different files:
// ex: generation of hashes    --> hash-generator.rs
//     organize the hashes     --> hash-organizer.rs
//     sort the hashes         --> hash-sorter.rs
//     store the hashes        --> store-file.rs

// ignore unused code
#[allow(dead_code)]
use blake3::Hasher;
use clap::{App, Arg};
use std::time::Instant;

mod hash_sorter;
mod store_file;

fn main() {
    // the following lines of code are simply for CLAP to keep records of this application use
    let matches = App::new("Vault")
        .version("1.0")
        .about("Generates hashes for specified nonces using BLAKE3")
        .arg(
            Arg::with_name("nonces")
                .short('n') // you can change this flag to whatever you want nonces to represent
                .long("nonces")
                .takes_value(true) // there must be a number inputted
                .help("Number of nonces to generate hashes for"),
        )
        .arg(
            Arg::with_name("filename")
                .short('f') // you can change this flag to whatever you want filename to represent
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

    let mut hashes = Vec::new();

    // Start the timer
    let start = Instant::now();

    for nonce in 0..num_nonces {
        let mut hasher = Hasher::new();
        hasher.update(&nonce.to_be_bytes()); // convert nonce to bytes and hash it
        let hash = hasher.finalize();
        hashes.push((nonce, hash));
    }

    hash_sorter::sort_hashes(&mut hashes);
    // for (nonce, hash) in hashes {
    //     println!("Nonce: {} | {}", nonce, hash);
    // }

    match store_file::store_hashes(&hashes, output_file) {
        Ok(_) => println!("Hashes successfully written to {}", output_file),
        Err(e) => eprintln!("Error writing hashes to file: {}", e),
    }

    let duration = start.elapsed();
    println!("Generated {} in {:?}", num_nonces, duration);
}
