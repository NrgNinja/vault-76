use std::{
    cmp::Ordering,
    fs::File,
    io::{self, BufRead, BufReader},
    time::Instant,
};

use memmap2::Mmap;

use crate::{Record, RECORD_SIZE};

#[derive(Debug)]
struct FileIndexEntry {
    filename: String,
    start_hash: Vec<u8>,
    end_hash: Vec<u8>,
}

// Single-threaded lookup
pub fn lookup_hash_in_file(directory: &str, target_hash: &str) -> io::Result<Option<Record>> {
    let target_hash_bytes = hex::decode(target_hash).expect("Invalid hex string for target hash");
    let target_hash_arr: &[u8] = target_hash_bytes.as_slice();
    let target_hash_len = target_hash_bytes.len();

    let file_index = read_file_index(directory)?;

    for entry in file_index {
        // println!("Checking file: {}", entry.filename);

        // Truncate start_hash and end_hash to the length of target_hash_bytes
        let truncated_start_hash = &entry.start_hash[..target_hash_len];
        let truncated_end_hash = &entry.end_hash[..target_hash_len];

        if target_hash_arr >= truncated_start_hash && truncated_end_hash >= target_hash_arr {
            // add print statement to check if None does it still go inside of here
            // let start_looking_inside = Instant::now();
            let file_path = format!("{}/{}", directory, entry.filename);
            // println!("Opening file: {}", files_path);

            let file = File::open(file_path)?;

            let mmap = unsafe { Mmap::map(&file).expect("Could not memory-map the file") };
            let buffer: &[u8] = &mmap;

            // Calculate the number of records expected
            let num_records = buffer.len() / RECORD_SIZE;

            // Check if the buffer size is a multiple of the record size
            if buffer.len() % RECORD_SIZE != 0 {
                eprintln!(
                    "Warning: File {} size is not a multiple of record size.",
                    entry.filename
                );
                continue;
            }

            // Perform binary search
            let binary_search_start: Instant = Instant::now();
            if let Some(found_record) = binary_search(
                num_records,
                buffer,
                target_hash_len,
                &target_hash_bytes,
                // start_looking_inside,
            ) {
                let binary_search_duration = binary_search_start.elapsed();
                println!("Binary search took: {:?}", binary_search_duration);
                return Ok(Some(found_record));
            }

            // Parallel binary search
            // let parallel_binary_search_start: Instant = Instant::now();
            // if let Some(found_record) = parallel_binary_search(
            //     num_records,
            //     buffer,
            //     file_name,
            //     target_hash_len,
            //     &target_hash_bytes,
            // ) {
            //     let parallel_binary_search_duration = parallel_binary_search_start.elapsed();
            //     println!(
            //         "Parallel binary search took: {:?}",
            //         parallel_binary_search_duration
            //     );
            //     return Ok(Some(found_record));
            // }

            // Linear search for records with prefix match
            // results = linear_search(
            //     num_records,
            //     file_name,
            //     buffer,
            //     &entry,
            //     &mut results,
            //     &target_hash_bytes,
            //     start_looking_inside,
            // );

            // let looking_inside_duration = start_looking_inside.elapsed();
            // println!(
            //     "Traversing over the contents of a file took: {:?}",
            //     looking_inside_duration
            // );
        }

        // let entry_duration = start_entry.elapsed();
        // println!("Going over entry {:?} takes: {:?}", entry, entry_duration);
    }

    Ok(None)
}

fn read_file_index(directory: &str) -> io::Result<Vec<FileIndexEntry>> {
    let file_path = format!("{}/file_index.bin", directory);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut index_entries = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line_trimmed = line.trim();

        let parts: Vec<&str> = line_trimmed.split_whitespace().collect();
        if parts.len() == 3 {
            let filename = parts[0].to_string();
            let start_hash = hex::decode(parts[1]).expect("Invalid hex string in file index");
            let end_hash = hex::decode(parts[2]).expect("Invalid hex string in file index");
            index_entries.push(FileIndexEntry {
                filename,
                start_hash,
                end_hash,
            });
        } else {
            eprintln!("Skipping line due to incorrect format: {}", line_trimmed);
        }
    }

    Ok(index_entries)
}

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

fn binary_search(
    num_records: usize,
    buffer: &[u8],
    target_hash_len: usize,
    target_hash_bytes: &Vec<u8>,
    // start_looking_inside: Instant,
) -> Option<Record> {
    let mut left = 0;
    let mut right = num_records - 1;

    while left <= right {
        let mid = (left + right) / 2;
        let mid_record_start = mid * RECORD_SIZE;
        let mid_record_end = mid_record_start + RECORD_SIZE;
        let mid_record_bytes = &buffer[mid_record_start..mid_record_end];

        let mid_record: Record = match bincode::deserialize(mid_record_bytes) {
            Ok(record) => record,
            Err(_) => {
                eprintln!("Failed to deserialize record at index {}", mid);
                break;
            }
        };

        match mid_record.hash[..target_hash_len].cmp(&target_hash_bytes.as_slice()) {
            Ordering::Less => left = mid + 1,
            Ordering::Greater => {
                if mid == 0 {
                    break;
                };
                right = mid - 1
            }
            Ordering::Equal => {
                // let entry_string = entry.file_name().to_string_lossy().to_string();
                // println!("Record is in file: {:?}", entry_string);
                return Some(mid_record);

                //     let looking_inside_duration = start_looking_inside.elapsed();
                //     println!(
                //         "(mid_value) Traversing over the contents of a file took: {:?}",
                //         looking_inside_duration
                //     );
            }
        }
    }

    return None;
}

// Multi-threaded
// pub fn lookup_hash(directory: &str, target_hash: &str) -> io::Result<Option<Record>> {
//     let target_hash_bytes = hex::decode(target_hash).expect("Hash couldn't be converted to hex");

//     // Read the contents of the directory
//     let paths: Vec<_> = std::fs::read_dir(directory)?
//         .filter_map(Result::ok)
//         .map(|entry| entry.path())
//         .collect();

//     // Atomic flag to indicate if the record has been found
//     let record_found = Arc::new(AtomicBool::new(false));
//     // Mutex to store the found record
//     let found_record = Arc::new(Mutex::new(None));

//     // Process each file in parallel
//     paths.par_iter().for_each(|path| {
//         if record_found.load(AtomicOrdering::Relaxed) {
//             return; // Early exit if record is already found
//         }

//         let file = match File::open(path) {
//             Ok(f) => f,
//             Err(_) => return,
//         };

//         let mmap = unsafe { Mmap::map(&file).expect("Could not memory-map the file") };

//         let buffer = &mmap;
//         // if file.read_to_end(&mut buffer).is_err() {
//         //     return;
//         // }

//         let chunk_size = 32;
//         let chunks = buffer.chunks_exact(chunk_size);

//         for chunk in chunks {
//             if record_found.load(AtomicOrdering::Relaxed) {
//                 return; // Early exit if record is already found
//             }

//             let record: Record = match bincode::deserialize(chunk) {
//                 Ok(r) => r,
//                 Err(_) => return,
//             };

//             if record.hash == target_hash_bytes.as_slice() {
//                 let mut found = found_record.lock().unwrap();
//                 *found = Some(record);
//                 record_found.store(true, AtomicOrdering::Relaxed); // Set the flag to indicate the record has been found
//                 return;
//             }
//         }
//     });

//     let found = found_record.lock().unwrap();
//     Ok(*found)
// }
