use std::fs::OpenOptions;
use std::io::{self, Seek, SeekFrom, Write};

use crate::Record;

// pub fn create_sparse_file(filename: &str, size: u64) -> io::Result<()> {
//     let file = OpenOptions::new().write(true).create(true).open(filename)?;
//     file.set_len(size)?;
//     // println!("{:?}", file);
//     Ok(())
// }

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

    let index_path = "output/file_index.bin";
    let mut index_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(index_path)?;

    let first_hash = hex::encode(chunk.first().unwrap().hash);
    let last_hash = hex::encode(chunk.last().unwrap().hash);

    writeln!(index_file, "{} {} {}", filename, first_hash, last_hash)?;

    Ok(())
}
