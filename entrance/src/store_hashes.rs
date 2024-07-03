use std::fs::OpenOptions;
use std::io::{self, Seek, SeekFrom, Write};

use crate::Record;

pub fn store_hashes_chunk(chunk: &[Record], filename: &str, offset: u64) -> io::Result<()> {
    let path = format!("output/{}", filename);
    let mut file = OpenOptions::new().write(true).create(true).open(path)?;

    let record_size = 32;

    // Collect all serialized records into a single buffer
    let mut buffer: Vec<u8> = Vec::with_capacity(chunk.len() * record_size);
    for hash in chunk {
        let encoded: Vec<u8> = bincode::serialize(hash).expect("Failed to serialize hash");
        buffer.extend_from_slice(&encoded);
    }

    // Seek to the start position and write the entire buffer
    file.seek(SeekFrom::Start(offset))?;
    file.write_all(&buffer)?;

    Ok(())
}
