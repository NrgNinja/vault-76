use std::{
    fs::File,
    io::{self, BufReader},
};

use crate::{print_records, Record, FILE_INDEX, HASH_SIZE};

fn lookup_hash_in_file(
    file_path: &str,
    target_hash: String,
) -> io::Result<Option<(u64, std::string::String)>> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let target_hash_bytes = hex::decode(target_hash).expect("Failed to decode hex string");
    if target_hash_bytes.len() != HASH_SIZE {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid hash length",
        ));
    }

    let target_hash_array: [u8; HASH_SIZE] = target_hash_bytes
        .try_into()
        .expect("Failed to convert hash to array");

    while let Ok(record) = bincode::deserialize_from::<&mut BufReader<File>, Record>(&mut reader) {
        if record.hash == target_hash_array {
            let nonce_decimal = print_records::nonce_to_decimal(&record.nonce);
            let hash_hex = print_records::hash_to_string(&record.hash);
            return Ok(Some((nonce_decimal, hash_hex)));
        }
    }

    Ok(None)
}

pub fn lookup_hash(
    directory: &str,
    target_hash: &str,
) -> Result<std::option::Option<(u64, std::string::String)>, std::io::Error> {
    let target_hash = target_hash.to_string();
    // Access the in-memory index
    let index = FILE_INDEX.lock().unwrap();

    for (filename, first_hash, last_hash) in index.iter() {
        if *first_hash <= target_hash && target_hash <= *last_hash {
            let file_path = format!("{}/{}", directory, filename);
            return lookup_hash_in_file(&file_path, target_hash);
        }
    }

    Ok(None)
}
