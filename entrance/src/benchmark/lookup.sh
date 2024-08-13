#!/bin/bash

k=$1
hash_len=$2
output_dir="../../output"
csv_file="lookup_csv/lookup_${k}_${hash_len}.csv"

# Clear output directory and create CSV headers
rm -rf "${output_dir:?}"/*
echo "LookupTime(ms),IsExist" > "$csv_file"

# Generate file
./../../target/release/entrance -k "$k" -t 16

# Run lookup for 1000 random prefixes
for i in {1..1000}; do
    # Clear cache
    free >/dev/null && sync >/dev/null && sudo sh -c 'echo 3 > /proc/sys/vm/drop_caches' && free >/dev/null

    # Get a random hash prefix
    hash=$(python3 get_hash.py "$i" "$hash_len")

    # Capture the output of the lookup
    output=$(./../../target/release/entrance -l "$hash")

    # Determine if records were found
    if [[ "$output" == *"No records found"* ]]; then
        is_exist="false"
    else
        is_exist="true"
    fi

    # Extract lookup time from output
    lookup_time=$(echo "$output" | grep -oP 'Search duration: \K[^\s]*')

    # Write to CSV
    echo "${lookup_time}${is_exist}" >> "$csv_file"
done

# Call Python script to analyze the CSV file
python3 check_lookup.py "$csv_file"