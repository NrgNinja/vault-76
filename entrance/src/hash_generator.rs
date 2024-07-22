// // this file stores the hash generation process of the vault
use crate::{Record, HASH_SIZE};
use blake3::Hasher;
use std::convert::TryInto;

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
    let mut bits_processed = 0;

    for &byte in hash_bytes {
        let bits_to_take = (prefix_length - bits_processed).min(8); // calculates the number of bits to take from the current hash byte (goal is to take the entire prefix_length, but we cannot take more than 8 bits at a time = 1 byte)
        prefix <<= bits_to_take; // shift current prefix value to the left by bits_to_take bits
        prefix |= (byte >> (8 - bits_to_take)) as u64;
        bits_processed += bits_to_take;

        if bits_processed >= prefix_length {
            break;
        }
    }

    prefix &= (1u64 << prefix_length) - 1;

    // return a tuple containing our extracted prefix and the Record of each nonce/hash pair
    (
        prefix,
        Record {
            nonce: nonce_bytes[2..8].try_into().unwrap(),
            hash: hash_bytes[0..HASH_SIZE].try_into().unwrap(),
        },
    )
}
