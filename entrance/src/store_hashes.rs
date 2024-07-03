// this file writes the hashes to disk using multiple threads
use crate::Record;
use dashmap::DashMap;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{self, BufWriter, Seek, SeekFrom, Write};
use std::sync::Arc;

pub fn create_sparse_file(filename: &str, size: u64) -> io::Result<()> {
    let file = OpenOptions::new().write(true).create(true).open(filename)?;
    file.set_len(size)?;
    Ok(())
}

// varvara's method of using a sparse file to store hashes
pub fn store_hashes_dashmap(map: &DashMap<u64, Vec<Record>>, filename: &str) -> io::Result<()> {
    let mut cumulative_offset = 0u64;
    let mut key_offsets = HashMap::new();

    // calculate offsets for each key based on the actual data size.
    for entry in map.iter() {
        let data_size = entry.value().len() as u64 * 32; // Each record is 32 bytes
        key_offsets.insert(*entry.key(), cumulative_offset);
        cumulative_offset += data_size;
    }

    create_sparse_file(filename, cumulative_offset)?;

    let file = Arc::new(OpenOptions::new().write(true).open(filename)?);

    // parallel writing using pre-calculated offsets.
    map.par_iter().for_each(|entry| {
        let local_file = file.clone();
        let offset = key_offsets[entry.key()];

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
