// this file writes the hashes to disk using multiple threads
use crate::Record;
use dashmap::DashMap;
// use heapless::Vec as HeaplessVec;
// use rayon::prelude::*;
use std::fs::OpenOptions;
use std::io::{self, BufWriter, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::sync::RwLock;
// use std::time::Instant;
// use std::fs::File;

// const BUFFER_SIZE: usize = 128 * 1024; // 128 KB
const RECORD_SIZE: usize = 32; // 32 bytes

// // multi-threaded approach where threads write to different parts of the file
// pub fn store_hashes_optimized(
//     map: &DashMap<u64, Vec<Record>>,
//     filename: &str,
//     memory_limit_bytes: usize,
//     record_size: usize,
// ) -> io::Result<()> {
//     let path = PathBuf::from("./../../output").join(filename);
//     let keys_and_offsets = prepare_offsets(&map)?;

//     keys_and_offsets
//         .into_par_iter()
//         .try_for_each(|(key, offset, _length)| {
//             if let Some(records) = map.get(&key) {
//                 let mut local_file = OpenOptions::new().write(true).open(&path)?;
//                 local_file.seek(SeekFrom::Start(offset as u64))?;
//                 let mut local_writer = BufWriter::with_capacity(BUFFER_SIZE, &local_file);

//                 let chunks = records.value().chunks(memory_limit_bytes / record_size);
//                 for chunk in chunks {
//                     for record in chunk {
//                         let encoded = serialize_record(record)?;
//                         local_writer.write_all(&encoded)?;
//                     }
//                 }
//                 local_writer.flush()?;
//             }
//             Ok::<(), io::Error>(())
//         })?;

//     Ok(())
// }

// // function to calculate the total size of the file by counting each record in the DashMap
// pub fn calculate_total_size(map: &DashMap<u64, Vec<Record>>) -> u64 {
//     map.iter()
//         .map(|entry| entry.value().len() as u64 * 32)
//         .sum()
// }

// // function to prepare the offsets for each key in the DashMap
// pub fn prepare_offsets(map: &DashMap<u64, Vec<Record>>) -> io::Result<Vec<(u64, usize, usize)>> {
//     let mut keys: Vec<u64> = map.iter().map(|entry| *entry.key()).collect();
//     keys.par_sort_unstable(); // sort keys in parallel

//     // let path = PathBuf::from("output").join("metadata.bin");
//     // use the one below when you want to cargo run from the benchmark folder
//     let path = PathBuf::from("./../../output").join("metadata.bin");

//     let metadata_file = OpenOptions::new().write(true).create(true).open(path)?;
//     let mut metadata_writer = BufWriter::new(metadata_file);

//     let mut offsets = Vec::new();
//     let mut cumulative_offset = 0;

//     for key in keys {
//         if let Some(records) = map.get(&key) {
//             let size = records.len() * 32; // each record is 32 bytes
//             offsets.push((key, cumulative_offset, size));

//             // serialize and write the metadata for each key, logging the start of this section
//             let metadata = format!("{},{},{}\n", key, cumulative_offset, size);
//             metadata_writer.write_all(metadata.as_bytes())?;

//             cumulative_offset += size;
//         }
//     }
//     metadata_writer.flush()?;
//     Ok(offsets)
// }

pub fn flush_to_disk(
    records: &DashMap<usize, Vec<Record>>,
    filename: &str,
    offsets: &RwLock<Vec<usize>>,
) -> io::Result<()> {
    let path: PathBuf = PathBuf::from("./../../output").join(filename);
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)?;

    let mut writer = BufWriter::new(&file);

    for entry in records.iter() {
        let (prefix, records) = entry.pair();
        let mut offsets = offsets.write().unwrap(); // Acquire write lock on offsets
        let offset = offsets[*prefix]; // Get current offset for this bucket

        writer.seek(SeekFrom::Start(offset as u64))?; // Move to the correct position in the file

        for record in records {
            // println!("Writing record: {:?} with prefix: {}", record, prefix);
            // println!("Offsets vector before changing: {:?}", offsets[*prefix]);
            writer.write_all(&record.nonce)?;
            writer.write_all(&record.hash)?;
            offsets[*prefix] += RECORD_SIZE; // Update offset after writing
            // println!("Offsets vector: {:?}", offsets[*prefix]);
        }
    }
    writer.flush()?;
    file.sync_all()?;
    Ok(())
}
