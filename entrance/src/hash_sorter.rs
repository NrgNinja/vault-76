use std::{
    io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    sync::RwLock,
};

use crate::{NONCE_SIZE};

pub fn sort_hashes(
    path: &String,
    bucket_index: usize,
    bucket_size: usize,
    offsets: &RwLock<Vec<usize>>,
    record_size: usize,
) {
    let offsets = offsets.read().unwrap(); // Acquire write lock on offsets

    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&path)
        .expect("Error opening file");

    let start = offsets[bucket_index] as u64;
    let end = start + (bucket_size * record_size) as u64;

    let mut reader = BufReader::new(&file);
    reader
        .seek(SeekFrom::Start(start))
        .expect("Error seeking to start of bucket");

    let mut bucket_records = Vec::with_capacity(bucket_size);
    let mut buffer = vec![0; record_size];

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
    bucket_records.sort_unstable_by(|a, b| a[NONCE_SIZE..].cmp(&b[NONCE_SIZE..]));

    let mut writer = BufWriter::new(&file);

    writer
        .seek(SeekFrom::Start(start))
        .expect("Error seeking to start of bucket");

    for record in bucket_records {
        writer
            .write_all(&record)
            .expect("Error writing record to file");
    }

    writer.flush().expect("Error flushing writer");
}
