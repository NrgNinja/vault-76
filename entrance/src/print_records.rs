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
                // to print hashes in binary format instead of hex:
                // let hash_binary = &record
                //     .hash
                //     .iter()
                //     .map(|b| format!("{:08b}", b))
                //     .collect::<Vec<String>>()
                //     .join("");
                let hash_hex = hash_to_string(&record.hash);

                let mut prefix = 0u64;
                let mut bits_processed = 0;

                for &byte in &record.hash {
                    let bits_to_take = (6 - bits_processed).min(8); // calculates the number of bits to take from the current hash byte (goal is to take the entire prefix_length, but we cannot take more than 8 bits at a time = 1 byte)
                    prefix <<= bits_to_take; // shift current prefix value to the left by bits_to_take bits
                    prefix |= (byte >> (8 - bits_to_take)) as u64;
                    bits_processed += bits_to_take;

                    if bits_processed >= 6 {
                        break;
                    }
                }

                prefix &= (1u64 << 6) - 1;

                println!("{:<16} | {} | {}", nonce_decimal, hash_hex, prefix);
                counter += 1;
            }
            Err(_) => break,
        }
    }
    Ok(())
}

pub fn verify_records_sorted(expected_count: usize) -> io::Result<()> {
    let path = "../../output/output.bin";
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut last_hash = vec![0u8; HASH_SIZE]; // Initially the smallest possible hash
    let mut is_first = true;
    let mut record_count = 0;

    loop {
        match deserialize_from::<&mut BufReader<File>, Record>(&mut reader) {
            Ok(record) => {
                record_count += 1;

                if is_first {
                    last_hash = record.hash.to_vec();
                    is_first = false;
                } else {
                    if last_hash > record.hash.to_vec() {
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            format!("output.bin doesn't seem to be sorted correctly with hashes",),
                        ));
                    }
                    last_hash = record.hash.to_vec();
                }
            }
            Err(_) => break,
        }
    }

    if record_count != expected_count {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Expected {} records but found {}",
                expected_count, record_count
            ),
        ));
    }

    println!("output.bin is sorted correctly and contains the expected number of records.");
    Ok(())
}
