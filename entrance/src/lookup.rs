use std::{
    cmp::Ordering,
    fs::File,
    io::{self, BufRead, BufReader},
};

use memmap2::Mmap;

use crate::{Record, HASH_SIZE, NONCE_SIZE, RECORD_SIZE};

// Single-threaded lookup
pub fn lookup_hash_in_file(directory: &str, target_hash: &str) -> io::Result<Option<Record>> {
    let target_hash_bytes = hex::decode(target_hash).expect("Invalid hex string for target hash");
    let target_hash_arr: &[u8] = target_hash_bytes.as_slice();
    let target_hash_len = target_hash_bytes.len();

    let file_path = format!("{}/file_index.bin", directory);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let line_trimmed = line.trim();

        let parts: Vec<&str> = line_trimmed.split_whitespace().collect();
        let mut filename = String::new();
        let mut start_hash = Vec::new();
        let mut end_hash = Vec::new();

        if parts.len() == 3 {
            filename = parts[0].to_string();
            start_hash = hex::decode(parts[1]).expect("Invalid hex string in file index");
            end_hash = hex::decode(parts[2]).expect("Invalid hex string in file index");
        } else {
            eprintln!("Skipping line due to incorrect format: {}", line_trimmed);
        }

        // Truncate start_hash and end_hash to the length of target_hash_bytes
        let truncated_start_hash = &start_hash[..target_hash_len];
        let truncated_end_hash = &end_hash[..target_hash_len];

        if target_hash_arr >= truncated_start_hash && truncated_end_hash >= target_hash_arr {
            let file_path = format!("{}/{}", directory, filename);

            let file = File::open(file_path)?;
            let mmap = unsafe { Mmap::map(&file).expect("Could not memory-map the file") };
            let buffer: &[u8] = &mmap;

            // Calculate the number of records expected
            let num_records = buffer.len() / RECORD_SIZE;

            // Check if the buffer size is a multiple of the record size
            if buffer.len() % RECORD_SIZE != 0 {
                eprintln!(
                    "Warning: File {} size is not a multiple of record size.",
                    filename
                );
                continue;
            }

            // Perform binary search
            if let Some(found_record) =
                binary_search(num_records, buffer, target_hash_len, &target_hash_bytes)
            {
                return Ok(Some(found_record));
            }
            break;
        }
    }

    Ok(None)
}

fn binary_search(
    num_records: usize,
    buffer: &[u8],
    target_hash_len: usize,
    target_hash_bytes: &Vec<u8>,
) -> Option<Record> {
    let mut left = 0;
    let mut right = num_records - 1;

    while left <= right {
        let mid = (left + right) / 2;
        let mid_record_start = mid * RECORD_SIZE;
        let mid_record_end = mid_record_start + RECORD_SIZE;
        let mid_record_bytes = &buffer[mid_record_start..mid_record_end];

        let hash = <[u8; HASH_SIZE]>::try_from(&mid_record_bytes[NONCE_SIZE..])
            .expect("Failed to read hash");

        match hash[..target_hash_len].cmp(&target_hash_bytes.as_slice()) {
            Ordering::Less => left = mid + 1,
            Ordering::Greater => {
                if mid == 0 {
                    break;
                };
                right = mid - 1
            }
            Ordering::Equal => {
                let nonce = <[u8; NONCE_SIZE]>::try_from(&mid_record_bytes[..NONCE_SIZE])
                    .expect("Failed to read nonce");
                let found_record = Record { nonce, hash };

                return Some(found_record);
            }
        }
    }

    return None;
}

// fn read_file_index(directory: &str) -> io::Result<Vec<FileIndexEntry>> {
//     let file_path = format!("{}/file_index.bin", directory);
//     let file = File::open(file_path)?;
//     let reader = BufReader::new(file);

//     let mut index_entries = Vec::new();

//     for line in reader.lines() {
//         let line = line?;
//         let line_trimmed = line.trim();

//         let parts: Vec<&str> = line_trimmed.split_whitespace().collect();
//         if parts.len() == 3 {
//             let filename = parts[0].to_string();
//             let start_hash = hex::decode(parts[1]).expect("Invalid hex string in file index");
//             let end_hash = hex::decode(parts[2]).expect("Invalid hex string in file index");
//             index_entries.push(FileIndexEntry {
//                 filename,
//                 start_hash,
//                 end_hash,
//             });
//         } else {
//             eprintln!("Skipping line due to incorrect format: {}", line_trimmed);
//         }
//     }

//     Ok(index_entries)
// }

// fn linear_search(
//     num_records: usize,
//     file_name: &str,
//     buffer: &[u8],
//     entry: &DirEntry,
//     results: &mut Vec<Record>,
//     target_hash_bytes: &Vec<u8>,
//     // start_looking_inside: Instant,
// ) -> Vec<Record> {
//     for i in 0..num_records {
//         // let record_start = i * record_size;
//         // let record_end = record_start + record_size;
//         // let record_bytes = &buffer[record_start..record_end];
//         // println!(
//         //     "Checking record hash: {:?}",
//         //     &buffer[i * record_size..(i + 1) * record_size]
//         // );

//         let record: Record =
//             match bincode::deserialize(&buffer[i * RECORD_SIZE..(i + 1) * RECORD_SIZE]) {
//                 Ok(record) => record,
//                 Err(_) => {
//                     eprintln!(
//                         "Failed to deserialize record at index {} in file {}",
//                         i, file_name
//                     );
//                     continue;
//                 }
//             };

//         // Check if the current record's hash starts with the target hash prefix
//         if record.hash.starts_with(&target_hash_bytes) {
//             // let looking_inside_duration = start_looking_inside.elapsed();
//             // println!(
//             //     "Traversing over the contents of a file took: {:?}",
//             //     looking_inside_duration
//             // );

//             let entry_string = entry.file_name().to_string_lossy().to_string();
//             println!("Record is in file: {:?}", entry_string);
//             results.push(record);
//             // return Ok(Some(record));
//         }
//     }

//     results.to_vec()
// }

// fn parallel_binary_search(
//     num_records: usize,
//     buffer: &[u8],
//     file_name: &str,
//     target_hash_len: usize,
//     target_hash_bytes: &[u8],
// ) -> Option<Record> {
//     let chunk_size = 1024;
//     let num_chunks = (num_records + chunk_size - 1) / chunk_size;

//     (0..num_chunks).into_par_iter().find_map_any(|chunk_index| {
//         let mut left = chunk_index * chunk_size; // chunk_start
//         let mut right = (left + chunk_size).min(num_records) - 1; // chunk_end

//         while left <= right {
//             let mid = (left + right) / 2;
//             let mid_record_start = mid * RECORD_SIZE;
//             let mid_record_end = mid_record_start + RECORD_SIZE;
//             let mid_record_bytes = &buffer[mid_record_start..mid_record_end];

//             let mid_record: Record = match bincode::deserialize(mid_record_bytes) {
//                 Ok(record) => record,
//                 Err(_) => {
//                     eprintln!(
//                         "Failed to deserialize record at index {} in file {}",
//                         mid, file_name
//                     );
//                     break;
//                 }
//             };

//             match mid_record.hash[..target_hash_len].cmp(&target_hash_bytes[..]) {
//                 Ordering::Less => left = mid + 1,
//                 Ordering::Greater => {
//                     if mid == 0 {
//                         break;
//                     }
//                     right = mid - 1;
//                 }
//                 Ordering::Equal => return Some(mid_record),
//             }
//         }
//         None
//     })
// }
