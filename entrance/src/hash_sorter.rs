use crate::Record;

// this file will sort the hashes in a sequential pattern
pub fn sort_hashes(hashes: &mut Vec<Record>) {
    hashes.sort_by(|a, b| a.nonce.cmp(&b.nonce));
}
