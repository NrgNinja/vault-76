// this file adds the operation to look up hashes based on a specified prefix
use crate::{Record, OUTPUT_FOLDER};
use bincode;
use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::time::Instant;

const RECORD_SIZE: usize = 32; // 6 bytes for nonce + 26 bytes for hash

pub fn lookup_by_prefix(filename: &str, prefix: &str) -> io::Result<()> {
    let path = PathBuf::from(OUTPUT_FOLDER).join(filename);
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    println!("{:?}", reader.capacity());

    let total_size = reader.seek(SeekFrom::End(0))? as usize;
    println!("Total size: {}", total_size);
    let num_records = total_size / RECORD_SIZE;

    let start_time = Instant::now();
    let (records, seek_count) = binary_search_by_prefix(&mut reader, num_records, prefix)?;
    let duration = start_time.elapsed();

    if !records.is_empty() {
        println!("{:<16} | {:<64}", "Nonce (Decimal)", "Hash (Hex)");
        println!("{}", "-".repeat(88));
        for record in records {
            let nonce_decimal = nonce_to_decimal(&record.nonce);
            let hash_hex = hash_to_string(&record.hash);
            println!("{:<16} | {}", nonce_decimal, hash_hex);
        }
    } else {
        println!("No records found with the specified prefix '{}'", prefix);
    }

    println!(
        "Search duration: {:?}, Seek operations: {}",
        duration, seek_count
    );
    Ok(())
}

fn binary_search_by_prefix<R: Read + Seek>(
    reader: &mut R,
    num_records: usize,
    prefix: &str,
) -> io::Result<(Vec<Record>, usize)> {
    let mut low = 0;
    let mut high = num_records as isize - 1;
    let mut records = Vec::new();
    let mut seek_count = 0;

    while low <= high {
        let mid = (low + high) / 2;
        reader.seek(SeekFrom::Start((mid as usize * RECORD_SIZE) as u64))?;
        seek_count += 1;

        if let Some(record) = deserialize_next_record(reader)? {
            let hash_hex = hash_to_string(&record.hash);
            if hash_hex.starts_with(prefix) {
                records.push(record);
                collect_records(
                    reader,
                    mid + 1,
                    num_records,
                    prefix,
                    true,
                    &mut records,
                    &mut seek_count,
                )?;
                collect_records(
                    reader,
                    mid - 1,
                    num_records,
                    prefix,
                    false,
                    &mut records,
                    &mut seek_count,
                )?;
                break;
            } else if hash_hex < prefix.to_owned() {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
    }

    Ok((records, seek_count))
}

fn collect_records<R: Read + Seek>(
    reader: &mut R,
    start: isize,
    end: usize,
    prefix: &str,
    forward: bool,
    records: &mut Vec<Record>,
    seek_count: &mut usize,
) -> io::Result<()> {
    let mut current = start;
    while (forward && current < end as isize) || (!forward && current >= 0) {
        reader.seek(SeekFrom::Start((current as usize * RECORD_SIZE) as u64))?;
        *seek_count += 1;
        if let Some(record) = deserialize_next_record(reader)? {
            let hash_hex = hash_to_string(&record.hash);
            if hash_hex.starts_with(prefix) {
                if forward {
                    records.push(record);
                } else {
                    records.insert(0, record);
                }
            } else {
                break;
            }
        }
        current += if forward { 1 } else { -1 };
    }
    Ok(())
}

fn nonce_to_decimal(nonce: &[u8; 6]) -> u64 {
    nonce.iter().fold(0u64, |acc, &b| acc * 256 + b as u64)
}

fn hash_to_string(hash: &[u8; 26]) -> String {
    hash.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join("")
}

fn deserialize_next_record<R: Read>(reader: &mut R) -> io::Result<Option<Record>> {
    let mut buffer = vec![0u8; RECORD_SIZE];
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
