#!/bin/bash

num_threads=1
#torus
max_num_threads=32 #6 runs
#pi
#max_num_threads=4 #3 runs
memory=$((4 * 1024 * 1024)) #4MB minimum
max_memory=$((1024 * 1024 * 1024))  # 1GB #11 runs
k=25

# Specify the output directory
output_dir="../../output"

echo "threads,memory,hash_time,sort_time,sync_time">> "vault_csv/vault-76-k${k}.csv"

for (( t=num_threads; t<=$max_num_threads; t=$((t * 2)) ))
do
    for (( m=memory; m<=$max_memory; m=$((m * 2)) ))
    do
        # Remove the output file
        rm -rf "${output_dir:?}"/*

        # clear cache
        free >/dev/null && sync >/dev/null && sudo sh -c 'echo 3 > /proc/sys/vm/drop_caches' && free >/dev/null
        sudo sync
        sleep 5

        echo "$k $t $m"
  
        # Run the cargo command with the current value of k and pipe the output to a file
        echo -n "$t,$m," >> "vault_csv/vault-76-k${k}.csv"
        cargo run --release -- -k "$k" -t "$t" -m "$m" >> "vault_csv/vault-76-k${k}.csv"
    done
done
