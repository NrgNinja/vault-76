// this file will sort the hashes in a sequential pattern
pub fn sort_hashes(hashes: &mut Vec<(u64, blake3::Hash)>) {
    hashes.sort_by(|a: &(u64, blake3::Hash), b| a.1.as_bytes().cmp(b.1.as_bytes()));
}
