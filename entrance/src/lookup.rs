use std::{
    fs::File,
    io::{self, Read},
    sync::{Arc, Mutex},
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::Record;

// Single-threaded lookup
pub fn lookup_hash_in_file(directory: &str, target_hash: &str) -> io::Result<Option<Record>> {
    let target_hash_bytes = hex::decode(target_hash).expect("Invalid hex string for target hash");

    let entries = std::fs::read_dir(directory)?;

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name = file_name.to_str().unwrap();

        // Extract the hash range from the file name
        let parts: Vec<&str> = file_name.trim_end_matches(".bin").split('-').collect();
        if parts.len() != 2 {
            continue;
        }

        let start_hash = hex::decode(parts[0]).expect("Invalid hex string in file name");
        let end_hash = hex::decode(parts[1]).expect("Invalid hex string in file name");

        if target_hash_bytes >= start_hash && end_hash >= target_hash_bytes {
            let file_path = format!("{}/{}", directory, file_name);
            let mut file = File::open(file_path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            // Calculate the number of records expected
            let record_size = 32;
            let num_records = buffer.len() / record_size;

            // Check if the buffer size is a multiple of the record size
            if buffer.len() % record_size != 0 {
                eprintln!(
                    "Warning: File {} size is not a multiple of record size.",
                    file_name
                );
                continue;
            }

            // Deserialize and search for the target hash
            for i in 0..num_records {
                let record_start = i * record_size;
                let record_end = record_start + record_size;
                let record_bytes = &buffer[record_start..record_end];
                let record: Record = match bincode::deserialize(record_bytes) {
                    Ok(record) => record,
                    Err(_) => {
                        eprintln!(
                            "Failed to deserialize record at index {} in file {}",
                            i, file_name
                        );
                        continue;
                    }
                };

                if record.hash == target_hash_bytes.as_slice() {
                    return Ok(Some(record));
                }
            }
        }
    }
    // let file = File::open(directory)?;
    // let mut reader = BufReader::new(file);

    // let target_hash_bytes = hex::decode(target_hash).expect("Failed to decode hex string");
    // if target_hash_bytes.len() != HASH_SIZE {
    //     return Err(io::Error::new(
    //         io::ErrorKind::InvalidInput,
    //         "Invalid hash length",
    //     ));
    // }

    // let target_hash_array: [u8; HASH_SIZE] = target_hash_bytes
    //     .try_into()
    //     .expect("Failed to convert hash to array");

    // while let Ok(record) = bincode::deserialize_from::<&mut BufReader<File>, Record>(&mut reader) {
    //     if record.hash == target_hash_array {
    //         let nonce_decimal = print_records::nonce_to_decimal(&record.nonce);
    //         let hash_hex = print_records::hash_to_string(&record.hash);
    //         return Ok(Some((nonce_decimal, hash_hex)));
    //     }
    // }

    Ok(None)
}

pub fn lookup_hash(directory: &str, target_hash: &str) -> io::Result<Option<Record>> {
    let target_hash_bytes = hex::decode(target_hash).expect("Hash couldn't be converted to hex");

    // Read the contents of the directory
    let paths: Vec<_> = std::fs::read_dir(directory)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect();

    // Mutex to store the found record
    let found_record = Arc::new(Mutex::new(None));

    // Process each file in parallel
    paths.par_iter().for_each(|path| {
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(_) => return,
        };

        let mut buffer = Vec::new();
        if file.read_to_end(&mut buffer).is_err() {
            return;
        }

        let chunk_size = 32;
        let chunks = buffer.chunks_exact(chunk_size);

        for chunk in chunks {
            let record: Record = match bincode::deserialize(chunk) {
                Ok(r) => r,
                Err(_) => return,
            };

            if record.hash == target_hash_bytes.as_slice() {
                let mut found = found_record.lock().unwrap();
                *found = Some(record);
                return;
            }
        }
    });

    let found = found_record.lock().unwrap();
    Ok(*found)
}
