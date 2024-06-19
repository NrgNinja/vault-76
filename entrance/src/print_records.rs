// use std::fs::File;
// use std::io::{self, Read};
// use std::str;

// pub fn print_records(filename: &str) -> io::Result<()> {
//     // Open the file
//     let mut file = File::open(filename)?;

//     // Buffer to hold a single record (6 bytes for nonce and 26 bytes for hash)
//     let mut buffer = [0; 32];

//     fn from_reader(mut rdr: impl Read) -> io::Result<Self> {
//         let item1 = rdr.read_u8()?;
//         rdr.read_exact(buf);
//         let item2 = rdr.read_u16::<LittleEndian>()?;
//         let item3 = rdr.read_i32::<LittleEndian>()?;

//         Ok(Configuration {
//             item1,
//             item2,
//             item3,
//         })
//     }

//     loop {
//         // reads a single record from a file
//         let bytes_read = file.read_exact(&mut buffer)?;

//         // println!("{:?}", buffer);

//         // If we reached the end of the file, stop the loop
//         if bytes_read == 0 {
//             break;
//         }

//         // if less than 32 bytes are read, that means that we have an incomplete record
//         if bytes_read != 32 {
//             eprintln!("Incomplete record found in file");
//             break;
//         }

//         // Get hash and nonce from the buffer
//         let nonce = &buffer[..6];
//         println!("{:?}", nonce);

//         let hash = &buffer[6..32];
//         println!("{:?}", hash);

//         // convert hash and nonce into hex
//         // let nonce_hex = hex::encode(&nonce);

//         // Convert hash bytes to string (assuming it's a UTF-8 string)
//         let hash_str = match str::from_utf8(hash) {
//             Ok(v) => v,
//             Err(e) => {
//                 eprintln!("Invalid UTF-8 sequence: {}", e);
//                 continue;
//             }
//         };

//         println!("Nonce: {:?}, Hash: {}", nonce, hash_str);
//     }

//     Ok(())
// }

use bincode::deserialize_from;
use std::fs::File;
use std::io::{self, BufReader};

use crate::Record;

pub fn print_records(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    while let Ok(record) = deserialize_from::<&mut BufReader<File>, Record>(&mut reader) {
        println!("{:?}", record);
    }

    Ok(())
}
