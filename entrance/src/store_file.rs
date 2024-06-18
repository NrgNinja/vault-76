// this file will take the generated, sorted hashes and write them to a file
use std::fs::File;
use std::io::{self, Write};

use crate::Record;

pub fn store_hashes(records: &Vec<Record>, filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;

    for record in records {
        file.write_all(&record.nonce.to_be_bytes())?;
        file.write_all(&record.hash.as_bytes())?;
    }
    Ok(())
}
