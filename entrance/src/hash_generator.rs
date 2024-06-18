// this file will store the hash generation process of the vault

use blake3::{Hasher, Hash};

// pub fn generate_hashes(num_nonces: u64) -> Vec<(u64, Hash)> {
//     let mut hashes = Vec::new();
//     for nonce in 0..num_nonces {
//         let mut hasher = Hasher::new();
//         hasher.update(&nonce.to_be_bytes()); // convert nonce to bytes and hash it
//         let hash = hasher.finalize();
//         hashes.push((nonce, hash));
//     }
//     hashes
// }

pub fn generate_hash(nonce: u64) -> (u64, Hash) {
    let mut hasher = Hasher::new();
    hasher.update(&nonce.to_be_bytes());
    let hash = hasher.finalize();
    (nonce, hash)
}
