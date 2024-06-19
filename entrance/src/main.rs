// this file will hold the main driver of our vault codebase
use blake3::Hasher;
use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use std::time::Instant;

mod hash_sorter;
mod print_records;
mod store_file;

const NONCE_SIZE: usize = 6;
const HASH_SIZE: usize = 26;

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
        // .arg(
        //     Arg::with_name("print")
        //         .short('p') // you can change this flag to whatever you want filename to represent
        //         .long("print")
        //         .takes_value(true) // there must be a filename inputted
        //         .help("Number of records to print"),
        // )
        .arg(
            Arg::with_name("sorting_on")
                .short('s') // you can change this flag to whatever you want filename to represent
                .long("sorting_on")
                .takes_value(true) // there must be a filename inputted
                .help("Turn sorting on/off"),
        )
        .get_matches();

    let num_nonces = matches
        .value_of("nonces")
        .unwrap_or("10") // default value if none specified
        .parse::<u64>() // parse it into 64 bit unsigned int
        .expect("Please provide a valid number for nonces");

    let output_file = matches.value_of("filename").unwrap_or("output.bin");

    // Define a variable to store the number of hashes that user wants to print
    // let num_records_to_print = matches
    //     .value_of("print")
    //     .unwrap_or("10")
    //     .parse::<u64>()
    //     .expect("Please provide a valid number of records to print");

    let sorting_on = matches
        .value_of("sorting_on")
        .unwrap_or("true")
        .parse::<bool>()
        .expect("Please provide a valid value for sorting_on (true/false)");

    let mut hashes: Vec<Record> = Vec::new();

    // Start the timer
    let start = Instant::now();

    for nonce in 0..num_nonces {
        // convert nonce to 6-byte array
        let nonce_bytes = (nonce as u64).to_be_bytes();
        let nonce_6_bytes: [u8; NONCE_SIZE] = nonce_bytes[2..8].try_into().unwrap(); // extract the lower 6 bytes as u8 array

        let mut hasher = Hasher::new();
        hasher.update(&nonce_6_bytes); // generate hash
        let hash = hasher.finalize();
        let hash = hash.to_string();
        let hash_slice = &hash[0..HASH_SIZE];
        let hash_slice = String::from(hash_slice);

        // let nonce_hex = hex::encode(&nonce_6_bytes);
        // let nonce = &nonce_hex[NONCE_SIZE..nonce_hex.len()];
        // let nonce = String::from(nonce);

        hashes.push(Record {
            nonce,
            hash: hash_slice,
        });
    }

    // Calls a function that sorts hashes in memory
    if sorting_on {
        hash_sorter::sort_hashes(&mut hashes);
    }

    // Calls store_hashes function to serialize generated hashes into binary and store them on disk
    match store_file::store_hashes(&hashes, output_file) {
        Ok(_) => println!("Hashes successfully written to {}", output_file),
        Err(e) => eprintln!("Error writing hashes to file: {}", e),
    }

    // Calls print_records function to deserialize and print all of the records into command prompt
    match print_records::print_records(output_file) {
        Ok(_) => println!("Hashes successfully deserialized from {}", output_file),
        Err(e) => eprintln!("Error deserializing hashes: {}", e),
    }

    let duration = start.elapsed();
    println!("Generated {} in {:?}", num_nonces, duration);
}
