// // this file is the main driver of the vault 76 codebase

// // different files of the vault 76 toolkit:
// // generation of hashes    --> hash-generator.rs
// // organize the hashes     --> hash-organizer.rs
// // sort the hashes         --> hash-sorter.rs
// // store the hashes        --> store-file.rs

// // ignore unused code
// #[allow(dead_code)]
// use clap::{App, Arg};
// use std::sync::{Arc, Mutex};
// use std::thread;
// // use rayon::prelude::*;
// use std::time::Instant;

// mod hash_generator;
// mod hash_sorter;
// mod store_file;

// fn main() {
//     let matches = App::new("Vault")
//         .version("1.0")
//         .about("Generates hashes for specified nonces using BLAKE3")
//         .arg(
//             Arg::with_name("nonces")
//                 .short('n') // cmd line flag
//                 .long("nonces")
//                 .takes_value(true) // there must be a number inputted
//                 .help("Number of nonces to generate hashes for"),
//         )
//         .arg(
//             Arg::with_name("filename")
//                 .short('f') // cmd line flag
//                 .long("filename")
//                 .takes_value(true) // there must be a filename inputted
//                 .help("Output file to store the generated hashes"),
//         )
//         .arg(
//             Arg::with_name("threads")
//                 .short('t') // cmd line flag
//                 .long("threads")
//                 .takes_value(true)
//                 .default_value("1") // there must be threads specified
//                 .help("Number of threads to use for hash generation"),
//         )
//         .get_matches();

//     let num_nonces = matches
//         .value_of("nonces")
//         .unwrap_or("10") // default value if none specified
//         .parse::<usize>() // parse it into 64 bit unsigned int
//         .expect("Please provide a valid number for nonces");

//     let num_threads = matches
//         .value_of("threads")
//         .unwrap_or("4")
//         .parse::<usize>()
//         .expect("Please provide a valid number for threads");

//     // output file to store binary format of hashes
//     let output_file = matches.value_of("filename").unwrap_or("output.bin");

//     // shared resource among threads
//     let hashes = Arc::new(Mutex::new(Vec::new()));

//     // vector to hold the thread handles
//     let mut handles = vec![];

//     // start the timer
//     let start = Instant::now();

//     // spawning threads to use
//     for i in 0..num_threads {
//         let thread_hashes = hashes.clone();
//         handles.push(thread::spawn(move || {
//             println!("Thread {} has ID {:?}", i, thread::current().id()); // view the threads
//             let per_thread = (num_nonces + num_threads - 1) / num_threads; // ensure full coverage of nonces
//             let thread_nonce_start = i * per_thread;
//             let thread_nonce_end = std::cmp::min(thread_nonce_start + per_thread, num_nonces);

//             let mut local_hashes = Vec::new();
//             for nonce in thread_nonce_start..thread_nonce_end {
//                 local_hashes.extend(hash_generator::generate_hashes(nonce as u64));
//             }

//             let mut hashes = thread_hashes.lock().unwrap(); // lock once per thread after work is done
//             hashes.extend(local_hashes);
//         }));
//     }

//     // wait for all threads to finish
//     for handle in handles {
//         handle.join().unwrap();
//     }

//     // TODO: join the vectors together, or sort them independently

//     // lock the mutex one final time to sort and store hashes
//     let mut final_hashes = hashes.lock().unwrap();

//     // calls hash_sorter.rs
//     hash_sorter::sort_hashes(&mut final_hashes);

//     // // for viewing the generated hashes
//     // for (nonce, hash) in hashes {
//     //     println!("Nonce: {} | {}", nonce, hash);
//     // }

//     // calls store_file.rs
//     match store_file::store_hashes(&final_hashes, output_file) {
//         Ok(_) => println!("Hashes successfully written to {}", output_file),
//         Err(e) => eprintln!("Error writing hashes to file: {}", e),
//     }

//     // end the timer
//     let duration = start.elapsed();
//     println!("Generated {} in {:?}", num_nonces, duration);
// }

use clap::{App, Arg};
use rayon::prelude::*;
use std::time::Instant;

mod hash_generator;

fn main() {
    let matches = App::new("Vault")
        .version("1.0")
        .about("Generates hashes for specified nonces using BLAKE3")
        .arg(
            Arg::with_name("nonces")
                .short('n')
                .long("nonces")
                .takes_value(true)
                .help("Number of nonces to generate hashes for"),
        )
        .arg(
            Arg::with_name("threads")
                .short('t')
                .long("threads")
                .takes_value(true)
                .default_value("1")
                .help("Number of threads to use for hash generation"),
        )
        .get_matches();

    let num_nonces = matches
        .value_of("nonces")
        .unwrap_or("10")
        .parse::<usize>()
        .expect("Please provide a valid number for nonces");

    let num_threads = matches
        .value_of("threads")
        .unwrap_or("4")
        .parse::<usize>()
        .expect("Please provide a valid number for threads");

    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();

    let start = Instant::now();

    // generate hashes in parallel
    let _hashes: Vec<_> = (0..num_nonces)
        .into_par_iter()
        .map(|nonce| hash_generator::generate_hash(nonce as u64)) // Cast usize to u64 here
        .collect();

    // for (nonce, hash) in &hashes {
    //     println!("Nonce: {}, Hash: {}", nonce, hash);
    // }

    let duration = start.elapsed();
    println!("Generated {} nonces in {:?}", num_nonces, duration);
}
