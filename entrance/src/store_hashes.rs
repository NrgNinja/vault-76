// this file writes the hashes to disk using multiple threads
use crate::{Record, OUTPUT_FOLDER};
use dashmap::DashMap;
use std::fs::OpenOptions;
use std::io::{self, BufWriter, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::sync::RwLock;

pub fn flush_to_disk(
    records: &DashMap<usize, Vec<Record>>,
    filename: &str,
    offsets: &RwLock<Vec<usize>>,
    record_size: usize
) -> io::Result<()> {
    let path: PathBuf = PathBuf::from(OUTPUT_FOLDER).join(filename);
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .expect("Error opening file");

    let mut writer = BufWriter::new(&file);
    let mut offsets = offsets.write().unwrap(); // Acquire read lock on offsets

    for entry in records.iter() {
        let (prefix, records) = entry.pair();
        let offset = offsets[*prefix]; // Get current offset for this bucket

        writer.seek(SeekFrom::Start(offset as u64))?; // Seek to the start of the bucket

        // Write all records for this bucket
        for record in records {
            // println!("Writing record: {:?} with prefix: {} and offset: {}", record, prefix, offsets[*prefix]);
            // println!("Offsets vector before changing: {:?}", offsets[*prefix]);
            writer.write_all(&record.nonce)?;
            writer.write_all(&record.hash)?;
            offsets[*prefix] += record_size; // Update offset after writing
                                             // println!("Offsets vector: {:?}", offsets[*prefix]);
        }
        // Update the offset for this bucket after writing all records
        offsets[*prefix] = offset + records.len() * record_size; // Increment by the number of bytes written
    }

    writer.flush()?;
    Ok(())
}
