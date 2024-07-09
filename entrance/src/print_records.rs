// this file prints records in the DashMap to the command line when specified by the p flag
use crate::Record;
use dashmap::DashMap;

// converts nonce from byte array to a decimal value
fn nonce_to_decimal(nonce: &[u8; 6]) -> u64 {
    nonce.iter().fold(0u64, |acc, &b| acc * 256 + b as u64)
}

// converts hash from byte array to a hexadecimal string
fn hash_to_string(hash: &[u8; 26]) -> String {
    hash.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join("")
}

// function to deserialize and print a specified number of records from a DashMap
pub fn print_records_dashmap(map: &DashMap<u64, Vec<Record>>, num_records_to_print: usize) {
    println!("Here are the first {} records:", num_records_to_print);
    println!(
        "{:<16} | {:<10} | {:<64}",
        "Nonce (Decimal)", "Prefix (Hex)", "Hash (Hex)"
    );
    println!("{}", "-".repeat(100)); // Separator line

    let mut printed = 0;

    let mut keys_with_records: Vec<(u64, Vec<Record>)> = map
        .iter()
        .map(|entry| (*entry.key(), entry.value().clone()))
        .collect();

    // sort by keys to ensure records are printed in the order of their prefixes
    keys_with_records.sort_by_key(|k| k.0);

    'outer: for (prefix, records) in keys_with_records {
        for record in records {
            if printed >= num_records_to_print {
                break 'outer;
            }
            let nonce_decimal = nonce_to_decimal(&record.nonce);
            let hash_hex = hash_to_string(&record.hash);
            let prefix_hex = format!("{:04x}", prefix);
            println!("{:<16} | {:<12} | {}", nonce_decimal, prefix_hex, hash_hex);
            printed += 1;
        }
    }
}
