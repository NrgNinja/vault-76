#!/bin/bash

num_threads=1
#torus
max_num_threads=32 #6 runs
#pi
#max_num_threads=4 #3 runs
memory=$((1024 * 1024 * 4))            #4MB (excluded 1 and 2)
max_memory=$((1024 * 1024 * 1024)) # 1GB #11 runs
k=25
output_dir="../../output"
csv_file="vault_csv/vault76_k${k}.csv"

echo "threads,memory,hash_time,sort_time,sync_time" >"$csv_file"

for ((t = num_threads; t <= max_num_threads; t *= 2)); do
    for ((m = memory; m <= max_memory; m *= 2)); do
        # Remove the output file
        output_file="${output_dir}/output.bin"
        rm -f "$output_file"

        free >/dev/null && sync >/dev/null && sudo sh -c 'echo 3 > /proc/sys/vm/drop_caches' && free >/dev/null
        sudo sync
        sleep 1

        echo "$k $t $m"

        # Run the cargo command with the current value of k and pipe the output to a file
        output=$(./../../target/release/entrance -k "$k" -t "$t" -m "$m")

        echo "$t,$m,$output" >>"$csv_file"
    done
done
