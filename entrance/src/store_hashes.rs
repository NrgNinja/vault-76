use crate::Record;
use heapless::Vec as HeaplessVec;
use postcard::to_vec;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::{self, Write};

pub fn store_hashes_chunk(chunk: &[Record], filename: &str) -> io::Result<()> {
    let path = format!("output/{}", filename);
    let file = OpenOptions::new().write(true).create(true).open(path)?;

    // Collect all serialized records into a single buffer
    let mut writer = BufWriter::new(file);
    for hash in chunk {
        let encoded: HeaplessVec<u8, 32> = to_vec(hash).expect("Failed to serialize hash");
        writer
            .write_all(&encoded)
            .expect("Failed to write hash to writer");
    }

    writer.flush()?;

    Ok(())
}
