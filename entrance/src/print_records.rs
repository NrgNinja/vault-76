use bincode::deserialize_from;
use std::fs::File;
use std::io::{self, BufReader};

use crate::Record;

pub fn print_records(filename: &str, num_records_print: u64) -> io::Result<()> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    let mut counter = 0;

    while counter < num_records_print {
        match deserialize_from::<&mut BufReader<File>, Record>(&mut reader) {
            Ok(record) => {
                println!("{:?}", record);
                counter += 1;
            }
            Err(_) => break,
        }
    }

    Ok(())
}
