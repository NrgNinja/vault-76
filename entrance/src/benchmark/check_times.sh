#!/bin/bash

# Ensure the proper number of arguments is provided
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <k-value> <number-of-threads>"
    exit 1
fi

k="$1"
threads="$2"

# Specify the output directory
output_dir="../../output"

# Specify the file to save the results
results_file="times_${k}_${threads}t.csv"
echo "Gen&Writing,Sorting" > "$results_file"

# Run the script multiple times and collect the data
for n in {1..3}; do
    echo "Running iteration $n..."

    rm -rf "${output_dir:?}"/*
    free >/dev/null && sync >/dev/null && sudo sh -c 'echo 3 > /proc/sys/vm/drop_caches' && free >/dev/null

    # Call the generate_once.sh script and capture its output
    output=$(./generate_once.sh "$k" "$threads")

    # Extract Generation & Writing and Sorting times
    gen_write_time=$(echo "$output" | grep -oP 'Generation \& Writing took \K[\d\.]+')
    sort_time=$(echo "$output" | grep -oP 'Sorting took \K[\d\.]+')

    # Save the extracted times to the CSV file
    echo "$gen_write_time,$sort_time" >> "$results_file"
done

# Move the results file to a specific directory
mv "$results_file" stats/times/

echo "Data collection complete. Results saved to stats/times/$results_file"