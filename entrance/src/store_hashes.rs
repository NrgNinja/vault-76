// this file writes the hashes to disk using multiple threads
use crate::Record;
use dashmap::DashMap;
use heapless::Vec as HeaplessVec;
use postcard::to_vec;
// use postcard::to_io;
use rayon::prelude::*;
use std::fs::OpenOptions;
use std::io::{self, BufWriter, Seek, SeekFrom, Write};
use std::path::PathBuf;
// use std::time::Instant;

const BUFFER_SIZE: usize = 128 * 1024; // 128 KB

// multi-threaded approach where threads write to different parts of the file
pub fn store_hashes_optimized(map: &DashMap<u64, Vec<Record>>, filename: &str) -> io::Result<()> {
    let path = PathBuf::from("output").join(filename);
    let file_size = calculate_total_size(&map);

    // create a sparse file of a determined size
    let file = OpenOptions::new().write(true).create(true).open(&path)?;
    file.set_len(file_size)?;

    let keys_and_offsets = prepare_offsets(&map); // returns Vec<(u64, usize, usize)> where each tuple is (key, offset, length)

    // parallel write to different sections of the file
    keys_and_offsets
        .into_par_iter()
        .try_for_each(|(key, offset, length)| {
            let records = map.get(&key).unwrap();
            let mut local_file = OpenOptions::new().write(true).open(&path)?;
            local_file.seek(SeekFrom::Start(offset as u64))?;
            let mut local_writer = BufWriter::with_capacity(BUFFER_SIZE, local_file);

            let mut total_written = 0; // amount of data written; debugging purposes

            // write serialized records to a heapless vector and then to the file
            for record in records.value() {
                let encoded: HeaplessVec<u8, 32> =
                    to_vec(record).expect("Failed to serialize hash");
                if total_written + encoded.len() <= length {
                    local_writer.write_all(&encoded)?;
                    total_written += encoded.len();
                }
            }

            if total_written != length {
                return Err(io::Error::new(io::ErrorKind::Other, "Data length mismatch"));
            }

            local_writer.flush()?;
            Ok::<(), io::Error>(())
        })?;

    // let file = OpenOptions::new().write(true).open(&path)?;

    // keys_and_offsets
    //     .into_par_iter()
    //     .try_for_each(|(key, offset, _length)| {
    //         let records = map.get(&key).unwrap();
    //         let mut local_writer = BufWriter::with_capacity(BUFFER_SIZE, &file);
    //         local_writer.seek(SeekFrom::Start(offset as u64))?;

    //         for record in records.value() {
    //             to_io(record, &mut local_writer)
    //                 .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    //         }
    //         local_writer.flush()?;
    //         Ok::<(), io::Error>(())
    //     })?;

    Ok(())
}

// function to calculate the total size of the file by counting each record in the DashMap
pub fn calculate_total_size(map: &DashMap<u64, Vec<Record>>) -> u64 {
    map.iter()
        .map(|entry| entry.value().len() as u64 * 32)
        .sum()
}

// function to prepare the offsets for each key in the DashMap
pub fn prepare_offsets(map: &DashMap<u64, Vec<Record>>) -> Vec<(u64, usize, usize)> {
    let mut keys: Vec<u64> = map.iter().map(|entry| *entry.key()).collect();
    keys.par_sort_unstable(); // sort keys in parallel

    let mut offsets = Vec::new();
    let mut cumulative_offset = 0;

    for key in keys {
        if let Some(records) = map.get(&key) {
            let size = records.len() * 32; // each record is 32 bytes
            offsets.push((key, cumulative_offset, size));
            cumulative_offset += size;
        }
    }
    offsets
}
