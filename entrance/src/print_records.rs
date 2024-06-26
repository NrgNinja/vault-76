// use crate::Record;
// use bincode::deserialize_from;
// use std::fs::File;
// use std::io::{self, BufReader};

// fn nonce_to_decimal(nonce: &[u8; 6]) -> u64 {
//     let mut num: u64 = 0;
//     for &byte in nonce.iter() {
//         num = num * 256 + byte as u64;
//     }
//     num
// }

// fn hash_to_string(hash: &[u8; 26]) -> String {
//     hash.iter()
//         .map(|b| format!("{:02x}", b))
//         .collect::<Vec<String>>()
//         .join("")
// }

// // function to deserialize and print all of the records into command line
// pub fn print_records(filename: &str, num_records_print: u64) -> io::Result<()> {
//     let file = File::open(filename)?;
//     let mut reader = BufReader::new(file);

//     println!("Here are the first {num_records_print} records:");
//     println!("{:<20} | {:<64}", "Nonce (Decimal)", "Hash (Hex)");
//     println!("{}", "-".repeat(88)); // Creates a separator line

//     let mut counter = 0;

//     while counter < num_records_print {
//         match deserialize_from::<&mut BufReader<File>, Record>(&mut reader) {
//             Ok(record) => {
//                 let nonce_decimal = nonce_to_decimal(&record.nonce);
//                 let hash_hex = hash_to_string(&record.hash);
//                 println!("{:<20} | {}", nonce_decimal, hash_hex);
//                 counter += 1;
//             }
//             Err(_) => break,
//         }
//     }

//     Ok(())
// }

use crate::Record;
use dashmap::DashMap;

// Converts nonce from byte array to a decimal value
fn nonce_to_decimal(nonce: &[u8; 6]) -> u64 {
    nonce.iter().fold(0u64, |acc, &b| acc * 256 + b as u64)
}

// Converts hash from byte array to a hexadecimal string
fn hash_to_string(hash: &[u8; 26]) -> String {
    hash.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join("")
}

// Function to print a specified number of records from a DashMap
pub fn print_records_dashmap(map: &DashMap<u64, Vec<Record>>, num_records_to_print: usize) {
    println!("Here are the first {} records:", num_records_to_print);
    println!("{:<20} | {:<64}", "Nonce (Decimal)", "Hash (Hex)");
    println!("{}", "-".repeat(88)); // Creates a separator line

    let mut printed = 0;

    'outer: for record_vec in map.iter() {
        for record in record_vec.value() {
            let nonce_decimal = nonce_to_decimal(&record.nonce);
            let hash_hex = hash_to_string(&record.hash);

            println!("{:<20} | {}", nonce_decimal, hash_hex);
            printed += 1;
            if printed >= num_records_to_print {
                break 'outer;
            }
        }
    }
}
