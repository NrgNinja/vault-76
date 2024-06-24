use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::fs::File;
use std::io::{self, BufWriter, Write};

use crate::Record;

// Serializes records into binary and stores them in a file on disk
pub fn store_hashes(records: &Vec<Record>, filename: &str) -> io::Result<()> {
    let file = File::create(filename)?;
    let writer = BufWriter::new(file);

    // Multi-threaded implementation
    records.par_iter().try_for_each(|record| {
        let mut local_writer = BufWriter::new(writer.get_ref().try_clone()?); // each thread creates its own BufWriter instance; each thread has its own `File` handle reference for writing
        local_writer.write_all(&record.nonce)?;
        local_writer.write_all(&record.hash)?;
        local_writer.flush()?;
        Ok(())
    })

    // Single-threaded implementation
    // for record in records {
    //     let record_bytes = record.to_bytes();
    //     writer.write_all(&record_bytes)?;
    //     // serialize_into(&mut writer, record).unwrap();
    // }

    // writer.flush()?;

    // Ok(())
}
