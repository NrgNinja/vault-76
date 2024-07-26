use std::{
    io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    sync::RwLock,
};

use crate::{NONCE_SIZE, RECORD_SIZE};
use rayon::slice::ParallelSliceMut; // Add this line to import the ParallelSliceMut trait

pub fn sort_hashes(
    path: &String,
    bucket_index: usize,
    bucket_size: usize,
    offsets: &RwLock<Vec<usize>>,
) {
    let offsets = offsets.write().unwrap(); // Acquire write lock on offsets

    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&path)
        .expect("Error opening file");

    let start = offsets[bucket_index] as u64;
    let end = start + (bucket_size * RECORD_SIZE) as u64;

    let mut reader = BufReader::new(&file);
    reader
        .seek(SeekFrom::Start(start))
        .expect("Error seeking to start of bucket");

    let mut bucket_records = Vec::with_capacity(bucket_size);
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

    let start_sorting = std::time::Instant::now();

    // Sort the records in the current bucket
    bucket_records.par_sort_unstable_by(|a, b| a[NONCE_SIZE..].cmp(&b[NONCE_SIZE..]));

    let sorting_duration = start_sorting.elapsed();
    // println!(
    //     "Sorting bucket {} took: {:?}",
    //     bucket_index, sorting_duration
    // );

    let start_writing = std::time::Instant::now();

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

    let writing_duration = start_writing.elapsed();
    // println!(
    //     "Writing sorted bucket {} took: {:?}",
    //     bucket_index, writing_duration
    // );
}


