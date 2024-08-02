#!/bin/bash

num_threads=1
#torus
max_num_threads=32 #6 runs
#pi
#max_num_threads=4 #3 runs
memory=$((1024 * 1024)) #1MB
max_memory=$((1024 * 1024 * 1024))  # 1GB #11 runs
k=25
echo "threads,memory,hash_time,sort_time,sync_time">> "vault-76-k${k}.txt"

for (( t=num_threads; t<=$max_num_threads; t=$((t * 2)) ))
do
    for (( m=memory; m<=$max_memory; m=$((m * 2)) ))
    do
        # Remove the output file
        rm ../../output/output.bin
        echo "$k $t $m"
  
        # Run the cargo command with the current value of k and pipe the output to a file
        echo -n "$t,$m," >> "vault-76-k${k}.txt"
        cargo run --release -- -k "$k" -t "$t" -m "$m" >> "vault-76-k${k}.txt"
    done
done
