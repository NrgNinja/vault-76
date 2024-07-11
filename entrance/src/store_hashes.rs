use crate::Record;
use postcard::to_io;
use std::fs::{File, OpenOptions};
use std::io::BufWriter;
use std::io::{self, Write};

pub fn store_hashes_chunk(chunk: &[Record], filename: &mut File) -> io::Result<()> {
    // let start_create_file = std::time::Instant::now();
    // let path = format!("output/{}", filename);
    // let file = OpenOptions::new()
    //     .write(true)
    //     .create(true)
    //     .truncate(true)
    //     .open(path)?;

    // let create_file_duration = start_create_file.elapsed();
    // println!("Time to create file: {:?}", create_file_duration);

    let start_serialize = std::time::Instant::now();

    // Collect all serialized records into a single buffer
    let mut writer = BufWriter::new(filename);
    to_io(chunk, &mut writer).expect("Failed to serialize records");

    let serialize_duration = start_serialize.elapsed();
    println!("Time to serialize: {:?}", serialize_duration);

    let start_flush = std::time::Instant::now();
    writer.flush()?;
    let flush_durations = start_flush.elapsed();
    println!("Time to flush: {:?}", flush_durations);

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

// pub fn create_files() {
//     for chunk in hashes.chunks(chunk_size) {
//         let first_hash = hex::encode(chunk.first().unwrap().hash);
//         let last_hash = hex::encode(chunk.last().unwrap().hash);
//         let chunk_filename = format!("{}-{}.bin", first_hash, last_hash);

//         let path = format!("output/{}", chunk_filename);
//         let file = OpenOptions::new()
//             .write(true)
//             .create(true)
//             .truncate(true)
//             .open(&path)
//             .expect("Failed to create file");

//         file_handles.push((file, chunk_filename.clone()));
//         results.push((first_hash, last_hash, chunk_filename));
//     }
// }
