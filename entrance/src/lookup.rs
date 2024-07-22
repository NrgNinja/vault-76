// this file adds the operation to look up hashes based on a specified prefix
use crate::Record;
use bincode;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek, SeekFrom};
use std::path::PathBuf;
use std::time::Instant;

// change this to match the default prefix length
const PREFIX_LENGTH: usize = 2;

// this function reads the records from the output file, deserializes them and then prints them
pub fn lookup_by_prefix(filename: &str, prefix: &str) -> io::Result<()> {
    // let path = PathBuf::from("output").join(filename);
    // use the one below when you want to cargo run from the benchmark folder
    let path = PathBuf::from("./../../output").join(filename);
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut _count = 0;

    let start_time = Instant::now();
    let mut is_exist: bool = false;

    // TODO: we can later change this number, 4, to match the prefix length
    let (offset, size) = find_offset_for_prefix(&prefix[..(PREFIX_LENGTH * 2)])?;
    reader.seek(SeekFrom::Start(offset))?;

    // println!("{:<16} | {:<64}", "Nonce (Decimal)", "Hash (Hex)");
    // println!("{}", "-".repeat(88));

    let end_offset = offset + size;
    let mut current_offset = offset;

    // iterate through the prefix bucket only; exit if we reach the end of the bucket
    while current_offset < end_offset {
        match deserialize_next_record(&mut reader) {
            Ok(Some(record)) => {
                if record_matches_prefix(&record, prefix) {
                    _count += 1;
                    let _nonce_decimal = nonce_to_decimal(&record.nonce);
                    let _hash_hex = hash_to_string(&record.hash);
                    // println!("{:<16} | {}", nonce_decimal, hash_hex);
                    is_exist = true;
                }
                current_offset += std::mem::size_of::<Record>() as u64;
            }
            Ok(None) => {
                is_exist = false;
                break;
            },
            Err(e) => return Err(e),
        }
    }

    let duration = start_time.elapsed();
    // println!(
    //     "\nFound {} records in {:?} matching the prefix '{}'",
    //     count, duration, prefix
    // );

    println!("{},{:?},{}", prefix, duration, is_exist);

    Ok(())
}

// converts nonce from byte array to a decimal value
fn nonce_to_decimal(nonce: &[u8; 6]) -> u64 {
    nonce.iter().fold(0u64, |acc, &b| acc * 256 + b as u64)
}

// converts hash from byte array to a hexadecimal string
fn hash_to_string(hash: &[u8; 26]) -> String {
    hash.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join("")
}

// deserialize the next record from the reader
fn deserialize_next_record<R: io::Read>(reader: &mut R) -> io::Result<Option<Record>> {
    let mut buffer = vec![0u8; std::mem::size_of::<Record>()];
    match reader.read_exact(&mut buffer) {
        Ok(_) => {
            let record: Record = bincode::deserialize(&buffer)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            Ok(Some(record))
        }
        Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => Ok(None),
        Err(e) => Err(e),
    }
}

// check if the record matches the entire prefix specified
fn record_matches_prefix(record: &Record, prefix: &str) -> bool {
    let hash_hex = hash_to_string(&record.hash);
    hash_hex.starts_with(prefix)
}

// find the offset and size of the bucket for the given prefix
fn find_offset_for_prefix(prefix: &str) -> io::Result<(u64, u64)> {
    // let metadata_path = PathBuf::from("output").join("metadata.bin");
    // use the one below when you want to cargo run from the benchmark folder
    let metadata_path = PathBuf::from("./../../output").join("metadata.bin");
    let metadata_file = File::open(metadata_path)?;
    let mut metadata_reader = BufReader::new(metadata_file);

    let prefix_num = u64::from_str_radix(prefix, 16)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    let mut line = String::new();
    while metadata_reader.read_line(&mut line)? > 0 {
        let parts: Vec<&str> = line.trim().split(',').collect();
        if let Ok(key) = parts[0].parse::<u64>() {
            if key == prefix_num {
                let offset = parts[1].parse::<u64>().map_err(|_| {
                    io::Error::new(io::ErrorKind::InvalidData, "Invalid offset in metadata")
                })?;
                let size = parts[2].parse::<u64>().map_err(|_| {
                    io::Error::new(io::ErrorKind::InvalidData, "Invalid size in metadata")
                })?;
                return Ok((offset, size));
            }
        }
        line.clear();
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Prefix not found in metadata",
    ))
}
