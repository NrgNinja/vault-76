use crate::Record;
use rayon::prelude::*;

// sorts hashes in memory using rayon's parallel unstable sort
pub fn sort_hashes(hashes: &mut Vec<Record>) {
    hashes.par_sort_unstable_by(|a, b| a.hash.cmp(&b.hash));
}


