use crate::{Record, HASH_SIZE, NONCE_SIZE};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

fn nonce_to_decimal(nonce: &[u8; 6]) -> u64 {
    nonce.iter().fold(0, |acc, &byte| (acc << 8) | byte as u64)
}

fn hash_to_string(hash: &[u8; 26]) -> String {
    hash.iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<Vec<_>>()
        .join("")
}

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

        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        let record_size = NONCE_SIZE + HASH_SIZE;
        let num_records = buffer.len() / record_size;

        for i in 0..num_records {
            if counter >= num_records_print {
                break;
            }

            let start = i * record_size;
            let nonce = <[u8; NONCE_SIZE]>::try_from(&buffer[start..start + NONCE_SIZE])
                .expect("Failed to read nonce");
            let hash =
                <[u8; HASH_SIZE]>::try_from(&buffer[start + NONCE_SIZE..start + record_size])
                    .expect("Failed to read hash");

            let record = Record { nonce, hash };
            let nonce_decimal = nonce_to_decimal(&record.nonce);
            let hash_hex = hash_to_string(&record.hash);
            println!("{:<20} | {}", nonce_decimal, hash_hex);
            counter += 1;
        }

        if counter >= num_records_print {
            break;
        }
    }

    Ok(())
}

// Prints contents of file_index that contains metadata about each file
pub fn print_index_file(index_file_path: &str) -> io::Result<()> {
    let file = File::open(index_file_path)?;
    let reader = io::BufReader::new(file);

    println!("Contents of file_index.bin:");

    for line in reader.lines() {
        println!("{}", line?);
    }

    println!("Done printing file_index.bin");

    Ok(())
}
