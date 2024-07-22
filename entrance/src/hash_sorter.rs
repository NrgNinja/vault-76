use std::{io::{BufReader, Read, Seek, SeekFrom, Write}, sync::RwLock};

use crate::{NONCE_SIZE, RECORD_SIZE};

pub fn sort_hashes(path: &String, bucket_index: usize, sort_memory: usize, offsets: &RwLock<Vec<usize>>) {
    let offsets = offsets.write().unwrap(); // Acquire write lock on offsets
    
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&path)
        .expect("Error opening file");

    let start = offsets[bucket_index] as u64;
    let end = start + sort_memory as u64;

    let mut reader = BufReader::new(&file);
    reader
        .seek(SeekFrom::Start(start))
        .expect("Error seeking to start of bucket");

    let mut bucket_records = Vec::with_capacity(sort_memory);
    let mut buffer = vec![0; RECORD_SIZE];

    while reader
        .stream_position()
        .expect("Error getting stream position")
        < end
    {
        if let Ok(_) = reader.read_exact(&mut buffer) {
            bucket_records.push(buffer.clone());
        } else {
            break;
        }
    }

    // Sort the records in the current bucket
    bucket_records.sort_by(|a, b| a[NONCE_SIZE..].cmp(&b[NONCE_SIZE..]));

    file.seek(SeekFrom::Start(start))
        .expect("Error seeking to start of bucket");
    for record in bucket_records {
        file.write_all(&record)
            .expect("Error writing record to file");
    }
}
