// this file stores the hash generation process of the vault
use crate::Record;
use blake3::Hasher;

const NONCE_SIZE: usize = 6;
const HASH_SIZE: usize = 26;

// multi-threaded approach - bytes
pub fn generate_hash(nonce: u64) -> Record {
    let nonce_bytes = (nonce).to_be_bytes();
    let nonce_6_bytes: [u8; NONCE_SIZE] = nonce_bytes[2..8].try_into().unwrap(); // extract the lower 6 bytes as u8 array

    let mut hasher = Hasher::new();
    hasher.update(&nonce_6_bytes); // generate hash
    let hash = hasher.finalize();
    let hash_slice = {
        let mut bytes = [0u8; HASH_SIZE];
        bytes.copy_from_slice(&hash.as_bytes()[..HASH_SIZE]);
        bytes
    };

    Record {
        nonce: nonce_6_bytes,
        hash: hash_slice,
    }
}

// Generate hashes in buckets and compare them based on prefix
pub fn generate_hash_bucket(bucket_index: usize, bucket_size: usize) -> Vec<Record> {
    let mut bucket = Vec::with_capacity(bucket_size);

    for nonce in (bucket_index * bucket_size)..(bucket_size * (bucket_index + 1) - 1) {
        let nonce_bytes = nonce.to_be_bytes();
        let nonce_6_bytes: [u8; NONCE_SIZE] = nonce_bytes[2..8].try_into().unwrap();

        let mut hasher = Hasher::new();
        hasher.update(&nonce_6_bytes);
        let hash = hasher.finalize();
        let hash_slice = {
            let mut bytes = [0u8; HASH_SIZE];
            bytes.copy_from_slice(&hash.as_bytes()[..HASH_SIZE]);
            bytes
        };

        let record = Record {
            nonce: nonce_6_bytes,
            hash: hash_slice,
        };

        bucket.push(record);
    }

    bucket
}

// Function to generate hashes for a batch of nonces and return a vector of (nonce, truncated hash) tuples
pub fn generate_hash_batch(start_nonce: u64, batch_size: u64) -> Vec<(u64, [u8; HASH_SIZE])> {
    let mut results = Vec::with_capacity(batch_size as usize);
    for nonce in start_nonce..start_nonce + batch_size {
        let mut hasher = Hasher::new();
        hasher.update(&nonce.to_be_bytes()); // Convert nonce to bytes and hash it
        let hash = hasher.finalize();
        let truncated_hash = {
            let mut bytes = [0u8; HASH_SIZE];
            bytes.copy_from_slice(&hash.as_bytes()[..HASH_SIZE]);
            bytes
        };

        results.push((nonce, truncated_hash));
    }
    results
}

// multi-threaded approach - string
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
