use crate::Record;

// Sorts hashes in memory 
pub fn sort_hashes(hashes: &mut Vec<Record>) {
    hashes.sort_by(|a, b| a.hash.cmp(&b.hash));
}
