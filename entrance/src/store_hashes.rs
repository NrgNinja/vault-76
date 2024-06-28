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

// use serde::Serialize;
use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};

use crate::Record;

pub async fn store_hashes(hashes: &[Record], filename: &str, _num_threads: &usize) -> io::Result<()> {
    let mut file = File::create(filename).await?;

    for record in hashes {
        let serialized = serde_json::to_string(record).unwrap();
        file.write_all(serialized.as_bytes()).await?;
        file.write_all(b"\n").await?;
    }

    file.flush().await?;
    Ok(())
}
