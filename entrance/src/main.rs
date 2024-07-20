// this file holds the main driver of our vault codebase
use clap::{App, Arg};
use dashmap::DashMap;
use hash_generator::generate_hash;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::f64;
use std::sync::RwLock;
use std::time::Instant;
use store_hashes::flush_to_disk;

mod hash_generator;
mod print_records;
mod store_hashes;

const RECORD_SIZE: usize = 32; // 6 bytes for nonce + 26 bytes for hash
const HASH_SIZE: usize = 26;
const NONCE_SIZE: usize = 6;

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    nonce: [u8; NONCE_SIZE], // nonce is always 6 bytes in size & unique; represented by an array of u8 6 elements
    hash: [u8; HASH_SIZE],
}

fn main() {
    // defines letters for arguments that the user can call from command line
    let matches = App::new("Vault")
        .version("2.0")
        .about("Generates hashes for unique nonces using BLAKE3 hashing function. This vault also has the ability to store each record (nonce/hash pair) into a vector, sort them accordingly, and even look them up efficiently.")
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
                .help("File size to be populated with hashes"))
        .arg(
            Arg::with_name("memory_limit")
                .short('m')
                .long("memory_limit")
                .takes_value(true)
                .help("Limit memory"),)
        .arg(
            Arg::with_name("prefix_length")
                .short('x')
                .long("prefix")
                .takes_value(true)
                .help("Specify the prefix length to extract from the hash"))
        .get_matches();

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

    let output_file = "output.bin";

    // libary to use multiple threads
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();

    let start_vault_timer = Instant::now();

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

    let ratio = ((file_size as f64) / (memory_size as f64)).ceil() as usize; // gives 0 if file_size < memory_size
    let mut write_size = if ratio > 0 {
        1024 * 1024 / ratio
    } else {
        1024 * 1024 // default to 1 MB if ratio is zero
    };

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
        prefix_size = (f64::log(num_buckets as f64, 2.0)).ceil() as usize;
        num_buckets = 2usize.pow(prefix_size as u32);
        prefix_size = (f64::log(num_buckets as f64, 2.0)).ceil() as usize;
        bucket_size = file_size / num_buckets; // disk bucket size (in bytes)
        expected_total_flushes = file_size / write_size;
        sort_memory = bucket_size * num_threads;

        // valid configuration
        if sort_memory <= memory_size && num_buckets >= 64 {
            println!("-----------------Found valid config------------------");
            write_size = memory_size / num_buckets;
            write_size = (write_size / 16) * 16;
            bucket_size = write_size * flush_size;
            memory_size = write_size * num_buckets;
            file_size = bucket_size * num_buckets;
            sort_memory = bucket_size * num_threads;
            num_records = file_size / RECORD_SIZE;
            expected_total_flushes = file_size / write_size;
            bucket_size = write_size * flush_size / RECORD_SIZE;

            println!("Memory size: {}", memory_size);
            println!("File size: {} bytes", file_size);
            println!("Write size (memory bucket size): {} bytes", write_size); // memory bucket size
            println!("Flush size: {}", flush_size); // how many times the flush happens
            println!("Disk bucket size (in records): {}", bucket_size); // Records in 1 disk bucket
            println!("Num buckets: {}", num_buckets);
            println!("Prefix size: {}", prefix_size);
            println!("Expected total flushes: {}", expected_total_flushes);
            println!("Sort memory: {} bytes", sort_memory);
            println!("Num records: {}", num_records);

            break;
        }

        write_size /= 2;
    }

    let map: DashMap<usize, Vec<Record>> = DashMap::with_capacity(num_buckets);

    let thread_memory_limit = if file_size < memory_size {
        file_size / num_threads
    } else {
        memory_size / num_threads // in bytes
    };

    let mut total_generated = 0;

    let mut offsets = vec![0; num_buckets];

    for i in 1..num_buckets {
        offsets[i] = offsets[i - 1] + bucket_size * 32;
    }

    let offsets_vector: RwLock<Vec<usize>> = RwLock::new(offsets);

    // println!("Offset vector: {:?}", offsets_vector);

    while total_generated < file_size {
        (0..num_threads).into_par_iter().for_each(|thread_index| {
            let mut local_size = 0;
            let mut nonce: u64 = (thread_index * (thread_memory_limit / RECORD_SIZE))
                .try_into()
                .unwrap();

            while local_size < thread_memory_limit {
                let (prefix, record) = generate_hash(nonce, prefix_size);

                nonce += 1;

                let mut records = map.entry(prefix as usize).or_insert_with(|| Vec::new());

                if records.len() >= write_size / 32 as usize {
                    continue;
                }
                records.push(record);
                local_size += RECORD_SIZE;

                // println!(
                //     "Records generated for prefix {}: {:?}",
                //     prefix,
                //     records.len()
                // );
            }
        });

        flush_to_disk(&map, &output_file, &offsets_vector).expect("Error flushing to disk");
        total_generated += thread_memory_limit * num_threads;
        map.clear();
    }

    // println!("Offsets vector: {:?}", offsets_vector);

    let duration = start_vault_timer.elapsed();
    print!("Generated");
    if !output_file.is_empty() {
        print!(", stored");
    }
    println!(" {} records in {:?}", num_records, duration);

    let offsets_vector_read = offsets_vector.read().unwrap(); // Use .unwrap() for simplicity in examples; handle errors as appropriate in production code
    println!(
        "Length of the offsets vector: {}",
        offsets_vector_read.len()
    );

    if num_records_to_print != 0 {
        match print_records::print_records_from_file(num_records_to_print) {
            Ok(_) => println!("Hashes successfully deserialized from {}", output_file),
            Err(e) => eprintln!("Error deserializing hashes: {}", e),
        }
    }
}
