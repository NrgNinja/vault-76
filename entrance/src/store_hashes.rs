// this file writes the hashes to disk using multiple threads
use crate::{Record, RECORD_SIZE};
use dashmap::DashMap;
// use rayon::prelude::*;
use std::fs::OpenOptions;
use std::io::{self, BufWriter, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::sync::RwLock;

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
    let mut offsets = offsets.write().unwrap(); // Acquire write lock on offsets

    for entry in records.iter() {
        let (prefix, records) = entry.pair();
        // let mut offsets = offsets.write().unwrap(); // Acquire write lock on offsets
        let offset = offsets[*prefix]; // Get current offset for this bucket
        // println!("Writing records for prefix: {} with offset: {}", prefix, offset);

        writer.seek(SeekFrom::Start(offset as u64))?; // Seek to the start of the bucket

        // Write all records for this bucket
        for record in records {
            // println!("Writing record: {:?} with prefix: {} and offset: {}", record, prefix, offsets[*prefix]);
            // println!("Offsets vector before changing: {:?}", offsets[*prefix]);
            writer.write_all(&record.nonce)?;
            writer.write_all(&record.hash)?;
            offsets[*prefix] += RECORD_SIZE; // Update offset after writing
            // println!("Offsets vector: {:?}", offsets[*prefix]);
        }
        // Update the offset for this bucket after writing all records
        offsets[*prefix] = offset + records.len() * RECORD_SIZE; // Increment by the number of bytes written
    }
    writer.flush()?;
    // file.sync_all()?;
    Ok(())
}
