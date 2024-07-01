// use crate::Record;
// use rayon::prelude::*;

// // sorts hashes in memory using rayon's parallel unstable sort
// pub fn sort_hashes(hashes: &mut Vec<Record>) {
//     hashes.par_sort_unstable_by(|a, b| a.hash.cmp(&b.hash));
// }

use crate::Record;
use rayon::prelude::*;

pub fn sort_hashes(hashes: &mut Vec<Record>) {
    let chunk_size = 100000; // Adjust chunk size as needed
    let num_chunks = (hashes.len() + chunk_size - 1) / chunk_size;

    // Sort chunks in parallel
    let mut sorted_chunks: Vec<Vec<Record>> = (0..num_chunks)
        .into_par_iter()
        .map(|i| {
            let start = i * chunk_size;
            let end = (start + chunk_size).min(hashes.len());
            let mut chunk: Vec<Record> = hashes[start..end].to_vec();
            chunk.sort_unstable_by(|a, b| a.hash.cmp(&b.hash));
            chunk
        })
        .collect();

    // Use a min-heap to merge sorted chunks
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut heap = BinaryHeap::new();
    for (chunk_idx, chunk) in sorted_chunks.iter_mut().enumerate() {
        if let Some(record) = chunk.pop() {
            heap.push((Reverse(record), chunk_idx));
        }
    }

    let mut sorted_hashes = Vec::with_capacity(hashes.len());

    while let Some((Reverse(record), chunk_idx)) = heap.pop() {
        sorted_hashes.push(record);
        if let Some(next_record) = sorted_chunks[chunk_idx].pop() {
            heap.push((Reverse(next_record), chunk_idx));
        }
    }

    // Copy sorted hashes back to the original vector
    hashes.copy_from_slice(&sorted_hashes);
}
