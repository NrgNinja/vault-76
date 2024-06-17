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

fn main() {
    // the following lines of code are simply for CLAP to keep records of this application use
    let matches = App::new("Vault")
        .version("1.0")
        .about("Generates hashes for specified nonces using BLAKE3")
        .arg(
            Arg::with_name("nonces")
                .short('n')  // you can change this flag to whatever you want nonces to represent
                .long("nonces")
                .takes_value(true)  // there must be a number inputted
                .help("Number of nonces to generate hashes for"),
        )
        .get_matches();

    let num_nonces = matches
        .value_of("nonces")
        .unwrap_or("10") // default value if none specified
        .parse::<u64>() // parse it into 64 bit unsigned int
        .expect("Please provide a valid number for nonces");

    for nonce in 0..num_nonces {
        let mut hasher = Hasher::new();
        hasher.update(&nonce.to_be_bytes()); // convert nonce to bytes and hash it
        let hash = hasher.finalize();
        println!("Nonce: {} | {}", nonce, hash);
    }
}
