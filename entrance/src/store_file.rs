use crate::Record;
use bincode::serialize_into;
use std::fs::File;
use std::io::{self, BufWriter};
use std::time::Instant;

// Serializes records into binary and stores them in a file on disk
pub fn store_hashes(records: &Vec<Record>, filename: &str) -> io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    let store_timer: Instant = Instant::now();
    for record in records {
        serialize_into(&mut writer, record).unwrap();
    }
    let store_finish = store_timer.elapsed();
    println!("Storing the hashes takes about: {:?}", store_finish);
    Ok(())
}
