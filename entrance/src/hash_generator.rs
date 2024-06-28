// this file stores the hash generation process of the vault
use crate::{Record, HASH_SIZE, NONCE_SIZE, PREFIX_LENGTH};
use blake3::Hasher;

// multi-threaded approach - bytes
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

// Generate hashes in buckets and compare them based on prefix
pub fn generate_hash_bucket(
    bucket_index: usize,
    bucket_size: usize,
    prefix: [u8; PREFIX_LENGTH],
) -> Vec<Record> {
    let mut bucket = Vec::with_capacity(bucket_size);
    let nonce = bucket_size * bucket_index;
    // let end_nonce = bucket_size * (bucket_index + 1);

    while bucket.len() != bucket_size {
        let nonce_bytes: [u8; NONCE_SIZE] = nonce.to_be_bytes()[2..8].try_into().unwrap();

        let mut hasher = Hasher::new();
        hasher.update(&nonce_bytes);
        let hash = hasher.finalize();
        let hash_slice = {
            let mut bytes = [0u8; HASH_SIZE];
            bytes.copy_from_slice(&hash.as_bytes()[..HASH_SIZE]);
            bytes
        };

        println!("{:?}", prefix);

        if hash_slice[0..PREFIX_LENGTH] == prefix {
            let record = Record {
                nonce: nonce_bytes,
                hash: hash_slice,
            };

            bucket.push(record);
        }
    }

    // for nonce in start_nonce..end_nonce {
    //     let nonce_bytes: [u8; NONCE_SIZE] = nonce.to_be_bytes()[2..8].try_into().unwrap(); // directly get the slice

    //     let mut hasher = Hasher::new();
    //     hasher.update(&nonce_bytes);
    //     let hash = hasher.finalize();
    //     let hash_slice = {
    //         let mut bytes = [0u8; HASH_SIZE];
    //         bytes.copy_from_slice(&hash.as_bytes()[..HASH_SIZE]);
    //         bytes
    //     };

    //     let record = Record {
    //         nonce: nonce_bytes,
    //         hash: hash_slice,
    //     };

    //     bucket.push(record);
    // }
    bucket
}

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
