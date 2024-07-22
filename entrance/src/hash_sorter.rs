use std::{io::{BufReader, Read, Seek, SeekFrom, Write}, sync::RwLock, time::Instant};

use crate::{NONCE_SIZE, RECORD_SIZE};

pub fn sort_hashes(path: &String, bucket_index: usize, bucket_size: usize, offsets: &RwLock<Vec<usize>>) {
    let start_open_file = Instant::now();

    let offsets = offsets.write().unwrap(); // Acquire write lock on offsets
    
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&path)
        .expect("Error opening file");

    let open_file_duration = start_open_file.elapsed();
    println!("Time to open file: {:?}", open_file_duration);

    let start_initialize_vars = Instant::now();

    let start = offsets[bucket_index] as u64;
    let end = start + (bucket_size * RECORD_SIZE) as u64;

    let mut reader = BufReader::new(&file);
    reader
        .seek(SeekFrom::Start(start))
        .expect("Error seeking to start of bucket");

    let mut bucket_records = Vec::with_capacity(bucket_size);
    let mut buffer = vec![0; RECORD_SIZE];

    let initialize_vars_duration = start_initialize_vars.elapsed();
    println!("Time to initialize variables: {:?}", initialize_vars_duration);

    let start_extracting_records = Instant::now();

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

    let extracting_records_duration = start_extracting_records.elapsed();
    println!("Time to extract records: {:?}", extracting_records_duration);

    let start_sorting_records = Instant::now();

    // Sort the records in the current bucket
    bucket_records.sort_by(|a, b| a[NONCE_SIZE..].cmp(&b[NONCE_SIZE..]));

    let sorting_records_duration = start_sorting_records.elapsed();
    println!("Time to sort records: {:?}", sorting_records_duration);

    let start_writing_records = Instant::now();

    file.seek(SeekFrom::Start(start))
        .expect("Error seeking to start of bucket");
    for record in bucket_records {
        file.write_all(&record)
            .expect("Error writing record to file");
    }

    let writing_records_duration = start_writing_records.elapsed();
    println!("Time to write records: {:?}", writing_records_duration);
}
