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
