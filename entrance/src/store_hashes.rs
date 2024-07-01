// Writes data to disk sequentially

// use rayon::iter::{IntoParallelIterator, ParallelIterator};
// use std::fs::File;
// use std::io::{self, BufWriter, Write};

// use crate::Record;

// // Serializes records into binary and stores them in a file on disk
// pub fn store_hashes(records: &Vec<Record>, filename: &str) -> io::Result<()> {
//     let file = File::create(filename)?;
//     let mut writer = BufWriter::new(file);

//     // Specify chunk size and splits records into chunks
//     let chunk_size = 2097152;
//     let record_chunks: Vec<&[Record]> = records.chunks(chunk_size).collect();

//     // Process chunks in parallel
//     let results: Vec<Vec<u8>> = record_chunks
//         .into_par_iter()
//         .map(|chunk| {
//             let mut buffer = Vec::with_capacity(chunk.len() * (32)); // pre-allocate buffer space

//             for record in chunk {
//                 buffer.extend_from_slice(&record.nonce);
//                 buffer.extend_from_slice(&record.hash);
//             }

//             buffer
//         })
//         .collect(); // collect all results from parallel processing into a vector

//     // Write results sequentially
//     for buffer in results {
//         writer.write_all(&buffer)?; // write buffer contents into the file
//     }

//     // Flush the writer to ensure all buffered data is written to disk
//     writer.flush()?;

//     Ok(())
// }

// Uses mutex and locks

// use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
// use std::fs::File;
// use std::io::{self, BufWriter, Seek, Write};
// use std::sync::Mutex;

// use crate::Record;

// // Serializes records into binary and stores them in a file on disk
// pub fn store_hashes(records: &[Record], filename: &str) -> io::Result<()> {
//     let file = File::create(filename)?;
//     let file = Mutex::new(file); // Use a Mutex to synchronize access to the file

//     // Specify chunk size and split records into chunks
//     let chunk_size = 268435456;
//     let record_chunks: Vec<&[Record]> = records.chunks(chunk_size).collect();

//     // Process chunks in parallel
//     record_chunks
//         .into_par_iter()
//         .enumerate()
//         .try_for_each::<_, io::Result<()>>(|(thread_num, chunk)| {
//             let mut buffer = Vec::with_capacity(chunk.len() * 32); // Pre-allocate buffer space

//             for record in chunk {
//                 buffer.extend_from_slice(&record.nonce);
//                 buffer.extend_from_slice(&record.hash);
//             }

//             let start_pos = (chunk_size * thread_num) as u64;
//             let mut local_file = file.lock().unwrap(); // Lock the file for writing
//             local_file.seek(io::SeekFrom::Start(start_pos))?;

//             local_file.write_all(&buffer)?;
//             Ok(())
//         })?;

//     Ok(())
// }

// Writes data to disk concurrently and uses BufWriter
// use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
// use std::fs::File;
// use std::io::{self, BufWriter, Seek, SeekFrom, Write};

// use crate::Record;

// // Serializes records into binary and stores them in a file on disk
// pub fn store_hashes(records: &[Record], filename: &str) -> io::Result<()> {
//     let file = File::create(filename)?;

//     // Calculate the size of each record
//     let record_size = std::mem::size_of::<Record>();

//     // Specify chunk size and split records into chunks
//     let chunk_size = 268435456;
//     let record_chunks: Vec<&[Record]> = records.chunks(chunk_size).collect();

//     // Process chunks in parallel
//     record_chunks
//         .into_par_iter()
//         .enumerate()
//         .try_for_each::<_, io::Result<()>>(|(num_thread, chunk)| {
//             let mut buffer = Vec::with_capacity(chunk.len() * record_size); // Pre-allocate buffer space

//             for record in chunk {
//                 buffer.extend_from_slice(&record.nonce);
//                 buffer.extend_from_slice(&record.hash);
//             }

//             // Calculate the start position for this chunk
//             let start_pos = (num_thread * chunk_size) as u64;

//             // Clone the file to create a new BufWriter for each thread
//             let local_file = file.try_clone()?;
//             let mut local_writer = BufWriter::new(local_file);

//             // Seek to the correct position and write the buffer
//             local_writer.seek(SeekFrom::Start(start_pos))?;
//             local_writer.write_all(&buffer)?;
//             local_writer.flush()?;

//             Ok(())
//         })?;

//     Ok(())
// }

// use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
// use std::fs::File;
// use std::io::{self, BufWriter, Seek, SeekFrom, Write};
// use std::sync::Arc;

// use crate::Record;

// // Serializes records into binary and stores them in a file on disk
// pub fn store_hashes(records: &[Record], filename: &str, num_threads: &usize) -> io::Result<()> {
//     let file = Arc::new(File::create(filename)?);

//     // Calculate the size of each record
//     let record_size = std::mem::size_of::<Record>();

//     // Specify chunk size and split records into chunks
//     let chunk_size = (records.len() / num_threads) * 32;
//     let record_chunks: Vec<&[Record]> = records.chunks(chunk_size).collect();

//     // Process chunks in parallel
//     record_chunks
//         .into_par_iter()
//         .enumerate()
//         .try_for_each::<_, io::Result<()>>(|(chunk_idx, chunk)| {
//             let mut buffer = Vec::with_capacity(chunk.len() * record_size); // Pre-allocate buffer space

//             for record in chunk {
//                 buffer.extend_from_slice(&record.nonce);
//                 buffer.extend_from_slice(&record.hash);
//             }

//             let start_pos = (chunk_idx * chunk_size) as u64;

//             let local_file = file.clone();
//             std::thread::spawn(move || {
//                 let mut local_writer = BufWriter::new(&*local_file);
//                 local_writer.seek(SeekFrom::Start(start_pos)).unwrap();
//                 local_writer.write_all(&buffer).unwrap();
//                 local_writer.flush().unwrap();
//             })
//             .join()
//             .unwrap();

//             Ok(())
//         })?;

//     Ok(())
// }

// single threaded write to disk approach
// use crate::Record;
// use dashmap::DashMap;
// use std::fs::File;
// use std::io::{BufWriter, Result, Write};

// // Function to serialize and store records from a DashMap into a binary file
// pub fn store_hashes_dashmap(map: &DashMap<u64, Vec<Record>>, filename: &str) -> Result<()> {
//     let file = File::create(filename)?;
//     let mut writer = BufWriter::new(file);

//     for record_vec in map.iter() {
//         for record in record_vec.value() {
//             writer.write_all(&record.nonce)?;
//             writer.write_all(&record.hash)?;
//         }
//     }

//     writer.flush()?;
//     Ok(())
// }

// sparse file method
// use crate::Record;
// use dashmap::DashMap;
// use std::fs::OpenOptions;
// use std::io::{self, BufWriter, Write, Seek, SeekFrom};
// use std::sync::Arc;
// use rayon::prelude::*;

// pub fn store_hashes_dashmap(map: &DashMap<u64, Vec<Record>>, filename: &str) -> io::Result<()> {
//     let file = Arc::new(OpenOptions::new().write(true).create(true).open(filename)?);
//     let num_threads = rayon::current_num_threads();
//     let estimated_chunk_size = map.len() * 32 / num_threads; // Simplified estimate

//     map.par_iter().for_each(|entry| {
//         let key = *entry.key();
//         let records = entry.value();
//         let offset = calculate_offset(&key, estimated_chunk_size);

//         let local_file = file.clone();
//         let mut local_writer = BufWriter::new(&*local_file);
//         local_writer.seek(SeekFrom::Start(offset)).unwrap();
        
//         for record in records.iter() {
//             local_writer.write_all(&record.nonce).unwrap();
//             local_writer.write_all(&record.hash).unwrap();
//         }

//         local_writer.flush().unwrap();
//     });

//     Ok(())
// }

// /// Calculates the file offset for a given key.
// fn calculate_offset(key: &u64, base_chunk_size: usize) -> u64 {
//     *key as u64 * base_chunk_size as u64
// }

// varvara's method of using a sparse file to store hashes
use crate::Record;
use dashmap::DashMap;
use std::fs::OpenOptions;
use std::io::{self, BufWriter, Write, Seek, SeekFrom};
use std::sync::Arc;
use rayon::prelude::*;

pub fn create_sparse_file(filename: &str, size: u64) -> io::Result<()> {
    let file = OpenOptions::new().write(true).create(true).open(filename)?;
    file.set_len(size)?;
    Ok(())
}

pub fn store_hashes_dashmap(map: &DashMap<u64, Vec<Record>>, filename: &str, num_threads: usize) -> io::Result<()> {
    let total_size = map.len() as u64 * 32; // Assuming each record takes up 32 bytes
    create_sparse_file(filename, total_size)?;

    let file = Arc::new(OpenOptions::new().write(true).open(filename)?);
    let chunk_size = total_size / num_threads as u64;

    map.par_iter().for_each(|entry| {
        let local_file = file.clone();
        let offset = *entry.key() as u64 * chunk_size;

        let mut local_writer = BufWriter::new(&*local_file);
        local_writer.seek(SeekFrom::Start(offset)).unwrap();

        for record in entry.value() {
            let encoded = bincode::serialize(record).expect("Failed to serialize hash");
            local_writer.write_all(&encoded).unwrap();
        }

        local_writer.flush().unwrap();
    });

    Ok(())
}
