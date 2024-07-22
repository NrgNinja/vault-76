// this file prints records specified by the command line flag: -p
use crate::{Record, HASH_SIZE};
use bincode::deserialize_from;
use std::fs::File;
use std::io::{self, BufReader};

// converts nonce from byte array to a decimal value
fn nonce_to_decimal(nonce: &[u8; 6]) -> u64 {
    nonce.iter().fold(0u64, |acc, &b| acc * 256 + b as u64)
}

// converts hash from byte array to a hexadecimal string
fn hash_to_string(hash: &[u8; HASH_SIZE]) -> String {
    hash.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join("")
}

// this function reads the records from the output file, deserializes them and then prints them
pub fn print_records_from_file(num_records_print: u64) -> io::Result<()> {
    let path = "./../../output/output.bin";
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    println!("{:<16} | {:<64}", "Nonce (Decimal)", "Hash (Hex)");
    println!("{}", "-".repeat(88)); // creates a separator line

    let mut counter = 0;

    while counter < num_records_print {
        match deserialize_from::<&mut BufReader<File>, Record>(&mut reader) {
            Ok(record) => {
                let nonce_decimal = nonce_to_decimal(&record.nonce);
                let hash_binary = &record
                    .hash
                    .iter()
                    .map(|b| format!("{:08b}", b))
                    .collect::<Vec<String>>()
                    .join("");
                let hash_hex = hash_to_string(&record.hash);
                println!("{:<16} | {}", nonce_decimal, hash_binary);
                counter += 1;
            }
            Err(_) => break,
        }
    }
    Ok(())
}

pub fn verify_records_sorted() -> io::Result<()> {
    let path = "./../../output/output.bin";
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut last_hash = vec![0u8; HASH_SIZE]; // Initially the smallest possible hash
    let mut is_first = true;

    loop {
        match deserialize_from::<&mut BufReader<File>, Record>(&mut reader) {
            Ok(record) => {
                if is_first {
                    last_hash = record.hash.to_vec();
                    is_first = false;
                } else {
                    if last_hash > record.hash.to_vec() {
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            "output.bin doesn't seem to be sorted correctly",
                        ));
                    }
                    last_hash = record.hash.to_vec();
                }
            }
            Err(_) => break,
        }
    }

    println!("output.bin seems to be sorted correctly");
    Ok(())
}
