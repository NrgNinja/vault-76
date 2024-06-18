use std::fs::File;
use std::io::{self, Read};

pub fn convert(filename: &str) -> io::Result<()> {
    // Open the file
    let mut file = File::open(filename)?;

    // Seek to the Nth byte from the start of the file (num_records * 32 is an offset)
    // file.seek(SeekFrom::Start(num_records * 32))?; -- if we want to get the Nth record

    // Read from the start of the file, first N * 32 bytes
    // let num_bytes: = num_records * 32;
    let mut buffer = [0; 32];
    file.read(&mut buffer)?;

    println!("Buffer: {:?}", buffer);

    Ok(())
}
