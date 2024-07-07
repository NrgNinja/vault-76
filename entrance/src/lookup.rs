use std::{
    cmp::Ordering,
    fs::{self, File},
    io,
    sync::{
        atomic::{AtomicBool, Ordering as AtomicOrdering},
        Arc, Mutex,
    },
    time::Instant,
};

use memmap2::Mmap;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::Record;

// Single-threaded lookup
pub fn lookup_hash_in_file(directory: &str, target_hash: &str) -> io::Result<Vec<Record>> {
    let target_hash_bytes = hex::decode(target_hash).expect("Invalid hex string for target hash");
    let target_hash_arr: &[u8] = target_hash_bytes.as_slice();
    let target_hash_len = target_hash_bytes.len();

    let mut results: Vec<Record> = Vec::new();

    // Check if the directory exists
    if !fs::metadata(directory).is_ok() {
        eprintln!("Directory does not exist: {}", directory);
        return Ok(results);
    }

    // Happens quickly - was checked with time()
    let entries = match std::fs::read_dir(directory) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Failed to read directory: {}", e);
            return Ok(results);
        }
    };

    for entry in entries {
        let start_entry = Instant::now();
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name = file_name.to_str().unwrap();

        // println!("Checking file: {}", file_name);

        // Extract the hash range from the file name
        let parts: Vec<&str> = file_name.trim_end_matches(".bin").split('-').collect();
        if parts.len() != 2 {
            continue;
        }

        let start_hash = hex::decode(parts[0]).expect("Invalid hex string in file name");
        let end_hash = hex::decode(parts[1]).expect("Invalid hex string in file name");

        // Truncate start_hash and end_hash to the length of target_hash_bytes
        let truncated_start_hash = &start_hash[..target_hash_len];
        let truncated_end_hash = &end_hash[..target_hash_len];

        if target_hash_arr >= truncated_start_hash && truncated_end_hash >= target_hash_arr {
            let start_looking_inside = Instant::now();
            let file_path = format!("{}/{}", directory, file_name);
            println!("Opening file: {}", file_path);

            let file = File::open(file_path)?;

            let mmap = unsafe { Mmap::map(&file).expect("Could not memory-map the file") };
            let buffer: &[u8] = &mmap;

            // Calculate the number of records expected
            let record_size = 32;
            let num_records = buffer.len() / record_size;

            // Check if the buffer size is a multiple of the record size
            if buffer.len() % record_size != 0 {
                eprintln!(
                    "Warning: File {} size is not a multiple of record size.",
                    file_name
                );
                continue;
            }

            // Perform binary search
            // let mut left = 0;
            // let mut right = num_records - 1;

            // while left <= right {
            //     let mid = (left + right) / 2;
            //     let mid_record_start = mid * record_size;
            //     let mid_record_end = mid_record_start + record_size;
            //     let mid_record_bytes = &buffer[mid_record_start..mid_record_end];

            //     // let left_record_start = left * record_size;
            //     // let left_record_end = left_record_start + record_size;
            //     // let left_record_bytes = &buffer[left_record_start..left_record_end];

            //     let mid_record: Record = match bincode::deserialize(mid_record_bytes) {
            //         Ok(record) => record,
            //         Err(_) => {
            //             eprintln!(
            //                 "Failed to deserialize record at index {} in file {}",
            //                 mid, file_name
            //             );
            //             break;
            //         }
            //     };

            //     // let left_record: Record = match bincode::deserialize(left_record_bytes) {
            //     //     Ok(record) => record,
            //     //     Err(_) => {
            //     //         eprintln!(
            //     //             "Failed to deserialize record at index {} in file {}",
            //     //             left, file_name
            //     //         );
            //     //         break;
            //     //     }
            //     // };

            //     // Check if the current record's hash starts with the target hash prefix
            //     if mid_record.hash[..target_hash_len].starts_with(&target_hash_bytes) {
            //         let looking_inside_duration = start_looking_inside.elapsed();
            //         println!(
            //             "Traversing over the contents of a file took: {:?}",
            //             looking_inside_duration
            //         );

            //         let entry_string = entry.file_name().to_string_lossy().to_string();
            //         println!("Record is in file: {:?}", entry_string);
            //         results.push(mid_record);
            //     }

            //     match mid_record.hash[..target_hash_len].cmp(&target_hash_bytes.as_slice()) {
            //         Ordering::Less => left = mid + 1,
            //         Ordering::Greater => {
            //             if mid == 0 {
            //                 break;
            //             };
            //             right = mid - 1
            //         }
            //         Ordering::Equal => {
            //             let looking_inside_duration = start_looking_inside.elapsed();
            //             println!(
            //                 "(mid_value) Traversing over the contents of a file took: {:?}",
            //                 looking_inside_duration
            //             );

            //             let entry_string = entry.file_name().to_string_lossy().to_string();
            //             println!("Record is in file: {:?}", entry_string);
            //             results.push(mid_record);
            //         }
            //     }

            // match left_record.hash.cmp(&target_hash_arr) {
            //     Ordering::Less => left += 1,
            //     Ordering::Greater => break,
            //     Ordering::Equal => {
            //         let looking_inside_duration = start_looking_inside.elapsed();
            //         println!(
            //             "(left_value) Traversing over the contents of a file took: {:?}",
            //             looking_inside_duration
            //         );
            //         let entry_string = entry.file_name().to_string_lossy().to_string();
            //         println!("Record is in file: {:?}", entry_string);
            //         return Ok(Some(left_record));
            //     }
            // }
            // }

            // Linear search for records with prefix match
            for i in 0..num_records {
                // let record_start = i * record_size;
                // let record_end = record_start + record_size;
                // let record_bytes = &buffer[record_start..record_end];
                // println!(
                //     "Checking record hash: {:?}",
                //     &buffer[i * record_size..(i + 1) * record_size]
                // );

                let record: Record =
                    match bincode::deserialize(&buffer[i * record_size..(i + 1) * record_size]) {
                        Ok(record) => record,
                        Err(_) => {
                            eprintln!(
                                "Failed to deserialize record at index {} in file {}",
                                i, file_name
                            );
                            continue;
                        }
                    };

                // Check if the current record's hash starts with the target hash prefix
                if record.hash.starts_with(&target_hash_bytes) {
                    let looking_inside_duration = start_looking_inside.elapsed();
                    println!(
                        "Traversing over the contents of a file took: {:?}",
                        looking_inside_duration
                    );

                    let entry_string = entry.file_name().to_string_lossy().to_string();
                    println!("Record is in file: {:?}", entry_string);
                    results.push(record);
                    // return Ok(Some(record));
                }
            }

            let looking_inside_duration = start_looking_inside.elapsed();
            println!(
                "Traversing over the contents of a file took: {:?}",
                looking_inside_duration
            );
        }

        let entry_duration = start_entry.elapsed();
        println!("Going over entry {:?} takes: {:?}", entry, entry_duration);
    }

    Ok(results)
}

// Multi-threaded
pub fn lookup_hash(directory: &str, target_hash: &str) -> io::Result<Option<Record>> {
    let target_hash_bytes = hex::decode(target_hash).expect("Hash couldn't be converted to hex");

    // Read the contents of the directory
    let paths: Vec<_> = std::fs::read_dir(directory)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect();

    // Atomic flag to indicate if the record has been found
    let record_found = Arc::new(AtomicBool::new(false));
    // Mutex to store the found record
    let found_record = Arc::new(Mutex::new(None));

    // Process each file in parallel
    paths.par_iter().for_each(|path| {
        if record_found.load(AtomicOrdering::Relaxed) {
            return; // Early exit if record is already found
        }

        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => return,
        };

        let mmap = unsafe { Mmap::map(&file).expect("Could not memory-map the file") };

        let buffer = &mmap;
        // if file.read_to_end(&mut buffer).is_err() {
        //     return;
        // }

        let chunk_size = 32;
        let chunks = buffer.chunks_exact(chunk_size);

        for chunk in chunks {
            if record_found.load(AtomicOrdering::Relaxed) {
                return; // Early exit if record is already found
            }

            let record: Record = match bincode::deserialize(chunk) {
                Ok(r) => r,
                Err(_) => return,
            };

            if record.hash == target_hash_bytes.as_slice() {
                let mut found = found_record.lock().unwrap();
                *found = Some(record);
                record_found.store(true, AtomicOrdering::Relaxed); // Set the flag to indicate the record has been found
                return;
            }
        }
    });

    let found = found_record.lock().unwrap();
    Ok(*found)
}
