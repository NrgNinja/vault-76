// this file holds the main driver of our vault codebase
use crate::progress_tracker::ProgressTracker;
use clap::{App, Arg};
use dashmap::DashMap;
use rand::random;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use spdlog::prelude::*;
use std::f64;
use std::sync::RwLock;
use std::time::{Duration, Instant};

mod hash_generator;
mod hash_sorter;
mod lookup;
mod print_records;
mod progress_tracker;
mod store_hashes;

const RECORD_SIZE: usize = 32; // 6 bytes for nonce + 26 bytes for hash
const HASH_SIZE: usize = 26;
const NONCE_SIZE: usize = 6;
const OUTPUT_FOLDER: &str = "../../output";

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    nonce: [u8; NONCE_SIZE], // nonce is always 6 bytes in size & unique; represented by an array of u8 6 elements
    hash: [u8; HASH_SIZE],
}

#[allow(unused_assignments)] // this is for expected_total_flushes not being read
fn main() {
    // defines letters for arguments that the user can call from command line
    let matches = App::new("Vault-76")
        .version("3.0")
        .about("Cryptographic hash tool that generates hashes for unique nonces using BLAKE3 hashing function. This vault also has the ability to store each record (nonce/hash pair) into a DashMap, by using a specified prefix as the key, and the record as a value. You can also look up records efficiently.")
        .arg(
            Arg::with_name("k-value")
                .short('k') // you can change this flag
                .long("k-value")
                .takes_value(true) // there must be a number inputted
                .help("Specify k value to compute 2^k nonces"),
        )
        .arg(
            Arg::with_name("print")
                .short('p')
                .long("print")
                .takes_value(true)
                .help("Number of records to print"),
        )
        .arg(
            Arg::with_name("sorting_on")
                .short('s')
                .long("sorting_on")
                .takes_value(true)
                .help("Turn sorting on/off"),
        )
        .arg(
            Arg::with_name("threads")
                .short('t')
                .long("threads")
                .takes_value(true)
                .default_value("1") 
                .help("Number of threads to use for hash generation"),
        )
        .arg(
            Arg::with_name("file_size")
                .short('f')
                .long("file_size")
                .takes_value(true)
                .default_value("0")
                .help("File size to be populated with hashes"),
        )
        .arg(
            Arg::with_name("memory_limit")
                .short('m')
                .long("memory_limit")
                .takes_value(true)
                .help("How much memory you want to limit for the vault to use"),
        )
        .arg(
            Arg::with_name("prefix_length")
                .short('x')
                .long("prefix")
                .takes_value(true)
                .help("Specify the prefix length to extract from the hash"),
        )
        .arg(
            Arg::with_name("verify")
                .short('v')
                .long("verify")
                .takes_value(false)  // automatically true if used, false otherwise
                .help("Verify that the hashes in the output file are in sorted order"),
        )
        .arg(
            Arg::with_name("lookup")
                .short('l')
                .long("lookup")
                .takes_value(true)
                .help("Lookup a record by a prefix"),
            )
        .arg(
            Arg::with_name("debug")
                .short('d')
                .long("debug")
                .takes_value(false)
                .help("Prints debug information to the command line")
            )
        .get_matches();

    let output_file = "output.bin";

    // determine if lookup is specified, otherwise continue normal vault operations
    if let Some(lookup_value) = matches.value_of("lookup") {
        if let Err(e) = lookup::lookup_by_prefix(output_file, lookup_value) {
            eprintln!("Error during lookup: {}", e);
        }
        return;
    }

    let k = matches
        .value_of("k-value")
        .unwrap_or("0")
        .parse::<u32>()
        .expect("Please provide a valid integer for k");

    let mut num_records = 2u64.pow(k) as usize;

    let num_threads = matches
        .value_of("threads")
        .unwrap_or("4")
        .parse::<usize>()
        .expect("Please provide a valid number for threads");

    let num_records_to_print = matches
        .value_of("print")
        .unwrap_or("0")
        .parse::<u64>()
        .expect("Please provide a valid number of records to print");

    let mut memory_size = matches
        .value_of("memory_limit")
        .unwrap_or("2147483648")
        .parse::<usize>()
        .expect("Please provide a valid number for memory_limit");

    let mut file_size = matches
        .value_of("file_size")
        .unwrap_or("0")
        .parse::<usize>()
        .expect("Please provide a valid number for file_size");

    let sorting_on = matches
        .value_of("sorting_on")
        .unwrap_or("true")
        .parse::<bool>()
        .expect("Please provide a valid boolean for sorting_on");

    let debug = matches.is_present("debug");

    let verify = matches.is_present("verify");

    // libary to use multiple threads
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();

    // if -f flag is not provided, calculate file size based on k value
    if file_size == 0 {
        file_size = num_records * RECORD_SIZE;
        // in bytes
    }

    // if memory_size is bigger than the file size of the hashes, set memory_size to file_size
    memory_size = if memory_size > file_size {
        file_size
    } else {
        memory_size
    };

    let ratio = ((file_size as f64) / (memory_size as f64)).ceil() as usize;
    let mut write_size = 1024 * 1024 / ratio;
    let mut flush_size;
    let mut bucket_size = 0;
    let mut num_buckets = 0;
    let mut prefix_size = 0;
    let mut expected_total_flushes;
    let mut sort_memory;

    // looking for optimal combination of prefix length, num of buckets, memory bucket size, and disk bucket size
    while write_size > 0 {
        flush_size = ratio;
        bucket_size = write_size * 1024 * flush_size;
        num_buckets = file_size / bucket_size;
        prefix_size = (num_buckets as f64).log(2.0).ceil() as u32 + 1;
        num_buckets = 2usize.pow(prefix_size as u32);
        prefix_size = (num_buckets as f64).log(2.0).ceil() as u32;
        bucket_size = file_size / num_buckets; // disk bucket size (in bytes)
        expected_total_flushes = file_size / write_size;
        sort_memory = bucket_size * num_threads;

        // valid configuration
        if sort_memory <= memory_size && num_buckets >= 64 {
            if debug {
                println!("-----------------Found valid config------------------");
            }
            write_size = memory_size / num_buckets;
            write_size = (write_size / 16) * 16;
            bucket_size = write_size * flush_size;
            memory_size = write_size * num_buckets;
            file_size = bucket_size * num_buckets;
            sort_memory = bucket_size * num_threads;
            num_records = file_size / RECORD_SIZE;
            expected_total_flushes = file_size / write_size;
            bucket_size = write_size * flush_size / RECORD_SIZE;

            if debug {
                println!(
                    "Memory size: {} bytes ({} GB)",
                    memory_size,
                    memory_size / 1024 / 1024 / 1024
                );
                println!(
                    "File size: {} bytes ({} GB)",
                    file_size,
                    file_size / 1024 / 1024 / 1024
                );
                println!(
                    "Write size [memory bucket size]: {} bytes ({} MB)",
                    write_size,
                    write_size / 1024 / 1024
                ); // memory bucket size
                println!("Flush size: {}", flush_size); // how many times the flush happens
                println!("Disk bucket size (in records): {}", bucket_size); // records in 1 disk bucket
                println!("Num buckets: {}", num_buckets);
                println!("Prefix size: {} bits", prefix_size);
                println!("Expected total flushes: {}", expected_total_flushes);
                println!(
                    "Sort memory: {} bytes ({} MB)",
                    sort_memory,
                    sort_memory / 1024 / 1024
                );
                println!("Number of records: {}", num_records);
            }

            break;
        }

        write_size /= 2;
    }

    let expected_flushes = file_size / write_size;

    if debug {
        info!("Opening Vault Entrance...");
    }
    // initialize tracker to track progress of vault operations
    // tracker outputs progress every 2 seconds (you can reduce this, but more redundant output lines)
    let tracker =
        ProgressTracker::new(num_records as u64, expected_flushes, Duration::from_secs(2));

    let start_vault_timer = Instant::now();

    let map: DashMap<usize, Vec<Record>> = DashMap::with_capacity(num_buckets);

    let thread_memory_limit = if file_size < memory_size {
        file_size / num_threads
    } else {
        memory_size / num_threads // in bytes
    };

    let mut total_generated = 0;

    // defining offset vector for the generation phase
    let mut offsets = vec![0; num_buckets];
    for i in 1..num_buckets {
        offsets[i] = offsets[i - 1] + bucket_size * RECORD_SIZE;
    }
    let offsets_vector: RwLock<Vec<usize>> = RwLock::new(offsets);

    let start_generation_writing = Instant::now();

    // generate hashes and write them to disk
    while total_generated < file_size {
        (0..num_threads).into_par_iter().for_each(|_thread_index| {
            let mut local_size = 0;
            let mut nonce: u64 = random();

            if debug {
                tracker.set_stage("[HASHGEN]");
            }
            while local_size < thread_memory_limit {
                let (prefix, record) = hash_generator::generate_hash(nonce, prefix_size as usize);

                nonce += 1;

                let mut records = map.entry(prefix as usize).or_insert_with(|| Vec::new());

                if records.len() >= write_size / 32 as usize {
                    continue;
                }
                records.push(record);
                local_size += RECORD_SIZE;
            }
            // completed a batch of records processed
            if debug {
                tracker.update_records_processed((local_size / RECORD_SIZE) as u64);
            }
        });

        store_hashes::flush_to_disk(&map, &output_file, &offsets_vector)
            .expect("Error flushing to disk");
        total_generated += thread_memory_limit * num_threads;

        if debug {
            let flush_increment = map.len();
            tracker.increment_flushes(flush_increment);
        }
        map.clear();
    }

    let generation_writing_duration = start_generation_writing.elapsed().as_secs_f64();
    // println!(
    //     "Generation & Writing took {:.2} seconds",
    //     generation_duration_in_seconds
    // );

    let mut sorting_duration = 0.0;
    let mut sync_duration = 0.0;

    if sorting_on {
        if debug {
            tracker.set_stage("[SORTING]");
            tracker.set_expected_flushes(num_buckets);
        }

        let start_sorting = Instant::now();

        // creating an offset vector for sorting
        let mut offsets = vec![0; num_buckets];
        for i in 1..num_buckets {
            offsets[i] = offsets[i - 1] + bucket_size * RECORD_SIZE;
        }
        let offsets_vector: RwLock<Vec<usize>> = RwLock::new(offsets);

        let path = format!("{}/{}", OUTPUT_FOLDER, output_file);

        let records_per_bucket = (num_records / num_buckets) as u64;

        // parallel processing of each bucket using rayon
        (0..num_buckets).into_par_iter().for_each(|bucket_index| {
            hash_sorter::sort_hashes(&path, bucket_index, bucket_size, &offsets_vector);
            if debug {
                tracker.update_records_processed(records_per_bucket);
                tracker.increment_flushes(1);
            }
        });

        sorting_duration = start_sorting.elapsed().as_secs_f64();
        // println!("Sorting took {:.2} seconds", sorting_duration_in_seconds);

        // sync the file and close it once done
        let file = std::fs::OpenOptions::new()
            .read(true)
            .open(&path)
            .expect("Error opening file");
        if debug {
            tracker.report_progress();
            tracker.set_stage("[SYNCING]");
        }
        let sync_timer = Instant::now();
        file.sync_data().expect("Error syncing data");
        sync_duration = sync_timer.elapsed().as_secs_f64();
        // println!("Syncing file took {:.2} seconds", sync_duration_in_seconds);
    }

    let mut duration_in_seconds = 0.0;
    let mut hashes_per_second = 0.0;
    let mut bytes_per_second = 0.0;

    if debug {
        let duration = start_vault_timer.elapsed();
        duration_in_seconds = duration.as_secs_f64();
        hashes_per_second = num_records as f64 / duration_in_seconds / 1_000_000.0; // convert to MH/s
        bytes_per_second = file_size as f64 / 1024.0 / 1024.0 / duration_in_seconds;
        // convert bytes to megabytes
    }

    println!(
        "{},{},{}",
        generation_writing_duration, sorting_duration, sync_duration
    );

    if debug {
        println!(
            "Completed {} GB vault [output.bin] in {:.2} seconds: {:.2} MH/s {:.2} MB/s",
            file_size / 1024 / 1024 / 1024,
            duration_in_seconds,
            hashes_per_second,
            bytes_per_second
        );
    }

    if num_records_to_print != 0 {
        match print_records::print_records_from_file(num_records_to_print) {
            Ok(_) => println!("Hashes successfully deserialized from {}", output_file),
            Err(e) => eprintln!("Error deserializing hashes: {}", e),
        }
    }

    if verify {
        match print_records::verify_records_sorted(num_records) {
            Ok(_) => println!("Verification successful."),
            Err(e) => println!("Verification failed: {}", e),
        }
    }
}
