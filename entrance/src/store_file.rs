use bincode::serialize_into;
use std::fs::File;
use std::io::{self, BufWriter};

use crate::Record;

// Serializes records into binary and stores them in a file on disk
pub fn store_hashes(records: &Vec<Record>, filename: &str) -> io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    for record in records {
        serialize_into(&mut writer, record).unwrap();
    }
    Ok(())
}
