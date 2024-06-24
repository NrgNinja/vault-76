use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::fs::File;
use std::io::{self, BufWriter, Write};

use crate::Record;

// Serializes records into binary and stores them in a file on disk
pub fn store_hashes(records: &Vec<Record>, filename: &str) -> io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    // Specify chunk size and splits records into chunks
    let chunk_size = 2097152;
    let record_chunks: Vec<&[Record]> = records.chunks(chunk_size).collect();

    // Process chunks in parallel
    let results: Vec<Vec<u8>> = record_chunks
        .into_par_iter()
        .map(|chunk| {
            let mut buffer = Vec::with_capacity(chunk.len() * (32));

            for record in chunk {
                buffer.extend_from_slice(&record.nonce);
                buffer.extend_from_slice(&record.hash);
            }

            buffer
        })
        .collect();

    // Write results sequentially
    for buffer in results {
        writer.write_all(&buffer)?;
    }

    writer.flush()?;

    Ok(())
}
