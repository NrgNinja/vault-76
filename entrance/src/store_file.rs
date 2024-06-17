// this file will take the generated, sorted hashes and write them to a file
use std::fs::File;
use std::io::{self, Write};
use blake3::Hash;

pub fn store_hashes(hashes: &[(u64, Hash)], filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    for (nonce, hash) in hashes {
        file.write_all(&nonce.to_be_bytes())?;
        file.write_all(hash.as_bytes())?;
    }
    Ok(())
}