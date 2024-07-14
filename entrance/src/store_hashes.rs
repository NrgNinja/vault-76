use crate::{Record, RECORD_SIZE};
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::{self, Write};
use std::process::Command;
use std::time::Instant;

pub fn store_hashes_chunk(chunk: &[Record], filename: &String) -> io::Result<()> {
    let path = format!("../../output/{}", filename);
    let path = path.as_str();

    let size = chunk.len() * RECORD_SIZE;

    let arg1 = format!("-s {}", size);

    let start_truncate_command = Instant::now();

    Command::new("truncate")
        .arg(arg1)
        .arg(path)
        .spawn()
        .expect("Failed to create output directory")
        .wait()
        .expect("Failed to create output directory");

    let truncate_command_duration = start_truncate_command.elapsed();
    println!("Truncate command took {:?}", truncate_command_duration);

    let start_open_file = Instant::now();

    let file = OpenOptions::new()
        .write(true)
        .open(path)
        .expect("Failed to open file");

    let open_file_duration = start_open_file.elapsed();
    println!("Open file took {:?}", open_file_duration);

    // Collect all serialized records into a single buffer
    let mut writer = BufWriter::new(file);

    // Writing directly (without using any serializer) is faster
    for record in chunk {
        writer
            .write_all(&record.nonce)
            .expect("Failed to write nonce");
        writer
            .write_all(&record.hash)
            .expect("Failed to write hash");
    }

    writer.flush()?;

    Ok(())
}

pub fn create_index_file(path: &str, results: Vec<(String, String, String)>) -> io::Result<()> {
    Command::new("truncate")
        .arg("-s 0")
        .arg(path)
        .spawn()
        .expect("Failed to create output directory")
        .wait()
        .expect("Failed to create output directory");

    let file = OpenOptions::new()
        .write(true)
        // .create(true)
        // .truncate(true)
        .open(path)?;

    let mut writer = BufWriter::new(file);

    for result in results {
        writer
            .write_all(format!("{} {} {}\n", result.2, result.0, result.1).as_bytes())
            .expect("Failed to write to index file");
    }
    writer.flush()?;

    Ok(())
}
