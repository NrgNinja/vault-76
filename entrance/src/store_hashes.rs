// this file writes the hashes to disk using multiple threads
use crate::Record;
use dashmap::DashMap;
// use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{self, BufWriter, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::sync::Arc;

pub fn create_sparse_file(filename: &str, size: u64) -> io::Result<()> {
    let file = OpenOptions::new().write(true).create(true).open(filename)?;
    file.set_len(size)?;
    Ok(())
}

// varvara's method of using a sparse file of a fixed size to store hashes (saves space)
pub fn store_hashes_dashmap(map: &DashMap<u64, Vec<Record>>, filename: &str) -> io::Result<()> {
    let mut path = PathBuf::from("output");
    path.push(filename);

    // collect and sort keys
    let mut keys: Vec<u64> = map.iter().map(|entry| *entry.key()).collect();
    keys.sort_unstable();

    // calculate cumulative offsets based on sorted keys
    let mut cumulative_offset = 0u64;
    let mut key_offsets = HashMap::new();
    for key in &keys {
        let data_size = map.get(key).map_or(0, |v| v.len() as u64 * 32); // each record is assumed to be 32 bytes
        key_offsets.insert(*key, cumulative_offset);
        cumulative_offset += data_size;
    }

    create_sparse_file(&path.to_string_lossy(), cumulative_offset)?;

    // use arc to share file across threads
    let file = Arc::new(OpenOptions::new().write(true).open(path)?);

    // write to file using sorted keys
    for key in keys {
        if let Some(records_ref) = map.get(&key) {
            let records = records_ref.value(); // Dereference to get the Vec<Record>
            let local_file = file.clone();
            let offset = key_offsets[&key];

            let mut local_writer = BufWriter::new(&*local_file);
            local_writer.seek(SeekFrom::Start(offset)).unwrap();

            // uncomment following to print records being written to file (debugging purposes)
            // println!(
            //     "Writing to offset {} for prefix {:04x} with {} records",
            //     offset,
            //     key,
            //     records.len()
            // );

            for record in records {
                let encoded = bincode::serialize(record).expect("Failed to serialize hash");
                local_writer.write_all(&encoded).unwrap();
            }
            local_writer.flush().unwrap();
        }
    }
    Ok(())
}
