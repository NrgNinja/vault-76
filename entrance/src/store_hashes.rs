use crate::{Record, RECORD_SIZE};
use heapless::Vec as HeaplessVec;
use postcard::to_vec;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::time::Instant;

pub fn store_hashes_chunk(chunk: &[Record], filename: &str) -> io::Result<()> {
    let path = format!("output/{}", filename);
    let mut file = OpenOptions::new().write(true).create(true).open(path)?;

    // Collect all serialized records into a single buffer
    let serialize_chunk_start = Instant::now();

    let mut buffer: Vec<u8> = Vec::with_capacity(chunk.len() * RECORD_SIZE);
    for hash in chunk {
        let encoded: HeaplessVec<u8, 32> = to_vec(hash).expect("Failed to serialize hash");
        // let encoded: Vec<u8> = bincode::serialize(hash).expect("Failed to serialize hash");
        buffer.extend_from_slice(&encoded)
    }
    let serialize_chunk_duration = serialize_chunk_start.elapsed();
    println!("Serializing took {:?}", serialize_chunk_duration);

    // Seek to the start position and write the entire buffer
    file.write_all(&buffer)?;

    Ok(())
}
