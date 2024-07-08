use crate::Record;
use bincode::deserialize_from;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn nonce_to_decimal(nonce: &[u8; 6]) -> u64 {
    let mut num: u64 = 0;
    for &byte in nonce.iter() {
        num = num * 256 + byte as u64;
    }
    num
}

pub fn hash_to_string(hash: &[u8; 26]) -> String {
    hash.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join("")
}

// function to deserialize and print all of the records into command line from a single file
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

// Print records from all the files stored in the directory
pub fn print_records(directory: &str, num_records_print: u64) -> io::Result<()> {
    let mut paths: Vec<_> = std::fs::read_dir(directory)?
        .filter_map(Result::ok)
        .collect();

    // Sort paths based on the filenames
    paths.sort_by_key(|dir_entry| dir_entry.file_name());

    println!("{}", "-".repeat(88));
    println!("Here are the first {num_records_print} records:");
    println!("{:<20} | {:<64}", "Nonce (Decimal)", "Hash (Hex)");
    println!("{}", "-".repeat(88)); // Creates a separator line

    let mut counter = 0;
    for path in paths {
        let file_path = path.path();
        let filename = file_path.file_name().unwrap().to_str().unwrap();

        // Skip files that don't follow the expected naming pattern
        if !filename.contains("-") || !filename.ends_with(".bin") {
            continue;
        }

        let file = File::open(file_path)?;
        let mut reader = BufReader::new(file);

        while counter < num_records_print {
            match deserialize_from::<&mut BufReader<File>, Record>(&mut reader) {
                Ok(record) => {
                    let nonce_decimal = nonce_to_decimal(&record.nonce);
                    let hash_hex = hash_to_string(&record.hash);
                    println!("{:<20} | {}", nonce_decimal, hash_hex);
                    counter += 1;
                }
                Err(_) => break,
            }
        }

        if counter >= num_records_print {
            break;
        }
    }

    Ok(())
}

pub fn print_index_file(index_file_path: &str) -> io::Result<()> {
    let file = File::open(index_file_path)?;
    let reader = io::BufReader::new(file);

    println!("Contents of file_index.bin:");

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
