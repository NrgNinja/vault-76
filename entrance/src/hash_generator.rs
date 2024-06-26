// // this file stores the hash generation process of the vault
// use crate::Record;
// use blake3::Hasher;

// const NONCE_SIZE: usize = 6;
// const HASH_SIZE: usize = 26;

// // multi-threaded approach - bytes
// pub fn generate_hash(nonce: u64) -> Record {
//     let nonce_bytes = (nonce).to_be_bytes();
//     let nonce_6_bytes: [u8; NONCE_SIZE] = nonce_bytes[2..8].try_into().unwrap(); // extract the lower 6 bytes as u8 array

//     let mut hasher = Hasher::new();
//     hasher.update(&nonce_6_bytes); // generate hash
//     let hash = hasher.finalize();
//     let hash_slice = {
//         let mut bytes = [0u8; HASH_SIZE];
//         bytes.copy_from_slice(&hash.as_bytes()[..HASH_SIZE]);
//         bytes
//     };

//     Record {
//         nonce: nonce_6_bytes,
//         hash: hash_slice,
//     }
// }

// multi-threaded approach - string
// pub fn generate_hash(nonce: u64) -> Record {
//     let nonce_bytes = (nonce).to_be_bytes();
//     let nonce_6_bytes: [u8; NONCE_SIZE] = nonce_bytes[2..8].try_into().unwrap(); // extract the lower 6 bytes as u8 array

//     let mut hasher = Hasher::new();
//     hasher.update(&nonce_6_bytes); // generate hash
//     let hash = hasher.finalize();

//     let hash = hash.to_string();
//     let hash_slice = &hash[0..HASH_SIZE];
//     let hash_slice = String::from(hash_slice);

//     Record {
//         nonce: nonce as u64,
//         hash: hash_slice,
//     }
// }

// // single threaded approach
// pub fn generate_hashes(num_records: u64) -> Vec<Record> {
//     let capacity = (num_records * 32).try_into().unwrap();
//     let mut hashes: Vec<Record> = Vec::with_capacity(capacity);
//     // let mut hashes: Vec<Record> = Vec::new();

//     for nonce in 0..num_records {
//         // convert nonce to 6-byte array
//         let nonce_bytes = (nonce as u64).to_be_bytes();
//         let nonce_6_bytes: [u8; NONCE_SIZE] = nonce_bytes[2..8].try_into().unwrap(); // extract the lower 6 bytes as u8 array

//         let mut hasher = Hasher::new();
//         hasher.update(&nonce_6_bytes); // generate hash
//         let hash = hasher.finalize();
//         let hash = hash.to_string();
//         let hash_slice = &hash[0..HASH_SIZE];
//         let hash_slice = String::from(hash_slice);

//         hashes.push(Record {
//             nonce,
//             hash: hash_slice,
//         });
//     }
//     println!("{}", hashes.len());

//     hashes
// }

// this method uses prefix extraction & returns the hash with its prefix for storage in DashMap
use crate::Record;
use blake3::Hasher;
use std::convert::TryInto;

const NONCE_SIZE: usize = 6;
const HASH_SIZE: usize = 26;
// const PREFIX_SIZE: usize = 12; // This should be configurable as per your requirements

// pub fn generate_hash(nonce: u64, prefix_length: usize) -> (u64, Record) {
//     let nonce_bytes = nonce.to_be_bytes();
//     let nonce_6_bytes: [u8; NONCE_SIZE] = nonce_bytes[2..8].try_into().unwrap();

//     let mut hasher = Hasher::new();
//     hasher.update(&nonce_6_bytes);
//     let hash = hasher.finalize();
//     let hash_bytes = hash.as_bytes();
//     let hash_slice: [u8; HASH_SIZE] = hash_bytes[0..HASH_SIZE].try_into().unwrap();

//     let prefix = hash_bytes[0..PREFIX_SIZE.min(prefix_length)]
//         .iter()
//         .fold(0u64, |acc, &b| (acc << 8) | b as u64);

//     (
//         prefix,
//         Record {
//             nonce: nonce_6_bytes,
//             hash: hash_slice,
//         },
//     )
// }

pub fn generate_hash(nonce: u64, prefix_length: usize) -> (u64, Record) {
    let nonce_bytes = nonce.to_be_bytes();
    let nonce_6_bytes: [u8; NONCE_SIZE] = nonce_bytes[2..8].try_into().unwrap();

    let mut hasher = Hasher::new();
    hasher.update(&nonce_6_bytes);
    let hash = hasher.finalize();
    let hash_bytes = hash.as_bytes();
    let hash_slice: [u8; HASH_SIZE] = hash_bytes[0..HASH_SIZE].try_into().unwrap();

    let prefix = hash_bytes[..prefix_length.min(8)]
        .iter()
        .fold(0u64, |acc, &b| (acc << 8) | b as u64);

    (
        prefix,
        Record {
            nonce: nonce_6_bytes,
            hash: hash_slice,
        },
    )
}
