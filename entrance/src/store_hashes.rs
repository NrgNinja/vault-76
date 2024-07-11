use crate::Record;
use postcard::to_io;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::{self, Write};

pub fn store_hashes_chunk(chunk: &[Record], filename: &str) -> io::Result<()> {
    let path = format!("output/{}", filename);
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    // Collect all serialized records into a single buffer
    let mut writer = BufWriter::new(file);
    to_io(chunk, &mut writer).expect("Failed to serialize records");

    writer.flush()?;

    Ok(())
}

pub fn create_index_file(path: &str, results: Vec<(String, String, String)>) -> io::Result<()> {
    let file = OpenOptions::new().write(true).create(true).open(path)?;

    let mut writer = BufWriter::new(file);

    for result in results {
        writer
            .write_all(format!("{} {} {}\n", result.2, result.0, result.1).as_bytes())
            .expect("Failed to write to index file");
    }

    Ok(())
}
