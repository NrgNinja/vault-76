// this file adds the operation to look up hashes based on a specified prefix
use crate::Record;
use bincode;
use std::fs::File;
use std::io::{self, BufReader, Seek, SeekFrom};
use std::path::PathBuf;
use std::time::Instant;

pub fn lookup_by_prefix(filename: &str, prefix: &str) -> io::Result<()> {
    let path = PathBuf::from("output").join(filename);
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut count = 0;

    let start_time = Instant::now();

    // assuming an index or efficient way to find the offset for this prefix
    let offset = find_offset_for_prefix(&mut reader, prefix)?;
    reader.seek(SeekFrom::Start(offset))?;

    // for easier human readability
    println!("{:<16} | {:<64}", "Nonce (Decimal)", "Hash (Hex)");
    println!("{}", "-".repeat(88));

    while let Some(record) = deserialize_next_record(&mut reader)? {
        if record_matches_prefix(&record, prefix) {
            count += 1;
            let nonce_decimal = nonce_to_decimal(&record.nonce);
            let hash_hex = hash_to_string(&record.hash);
            println!("{:<16} | {}", nonce_decimal, hash_hex);
        }
    }

    let duration = start_time.elapsed();
    println!(
        "\nFound {} records in {:?} matching the prefix '{}'",
        count, duration, prefix
    );

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

fn record_matches_prefix(record: &Record, prefix: &str) -> bool {
    let hash_hex = hash_to_string(&record.hash);
    hash_hex.starts_with(prefix)
}

fn find_offset_for_prefix<R: io::Read + io::Seek>(reader: &mut R, prefix: &str) -> io::Result<u64> {
    reader.seek(SeekFrom::Start(0))?; // Start from the beginning of the file
    let mut offset = 0;

    while let Some(record) = deserialize_next_record(reader)? {
        if record_matches_prefix(&record, prefix) {
            return Ok(offset);
        }
        offset += std::mem::size_of::<Record>() as u64;
    }

    Err(io::Error::new(io::ErrorKind::NotFound, "Prefix not found"))
}
