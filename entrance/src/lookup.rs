use std::{
    fs::{self, File},
    io::{self, BufReader},
};

use bincode::deserialize_from;

use crate::Record;

fn get_file_for_hash(directory: &str, target_hash: &[u8; 26]) -> Option<String> {
    let target_hash_hex = hex::encode(target_hash);

    let paths: Vec<_> = fs::read_dir(directory)
        .unwrap()
        .filter_map(Result::ok)
        .collect();

    for path in paths {
        let filename = path.file_name().into_string().unwrap();

        if let Some((first_hash, last_hash)) = filename.split_once('-') {
            let last_hash = last_hash.trim_end_matches(".bin");

            if first_hash <= target_hash_hex.as_str() && target_hash_hex.as_str() <= last_hash {
                return Some(path.path().to_string_lossy().into_owned());
            }
        }
    }

    None
}

fn lookup_hash_in_file(file_path: &str, target_hash: &[u8; 26]) -> io::Result<Option<Record>> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    while let Ok(record) = deserialize_from::<&mut BufReader<File>, Record>(&mut reader) {
        if record.hash == *target_hash {
            return Ok(Some(record));
        }
    }

    Ok(None)
}

pub fn lookup_hash(directory: &str, target_hash: &[u8; 26]) -> io::Result<Option<Record>> {
    if let Some(file_path) = get_file_for_hash(directory, target_hash) {
        lookup_hash_in_file(&file_path, target_hash)
    } else {
        Ok(None)
    }
}
