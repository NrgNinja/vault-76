// // this file stores the hash generation process of the vault
use crate::Record;
use blake3::Hasher;
use std::convert::TryInto;

// const NONCE_SIZE: usize = 6;
const HASH_SIZE: usize = 26;

// this method uses prefix extraction & returns the hash with its prefix for storage in DashMap
#[inline]
pub fn generate_hash(nonce: u64, prefix_length: usize) -> (u64, Record) {
    // convert the nonce and hash to byte arrays (6 and 26 respectively)
    let nonce_bytes = nonce.to_be_bytes();
    let mut hasher = Hasher::new();
    hasher.update(&nonce_bytes[2..8]); // extract the lower 6 bytes as u8 array
    let hash = hasher.finalize();
    let hash_bytes = hash.as_bytes();

    // prefix of desired length is extracted using bitshifting from left to right
    let mut prefix = 0u64;
    for i in 0..prefix_length.min(8) {
        prefix <<= 8;
        prefix |= hash_bytes[i] as u64;
    }

    // return a tuple containing our extracted prefix and the Record of each nonce/hash pair
    (
        prefix,
        Record {
            nonce: nonce_bytes[2..8].try_into().unwrap(),
            hash: hash_bytes[0..HASH_SIZE].try_into().unwrap(),
        },
    )
}
