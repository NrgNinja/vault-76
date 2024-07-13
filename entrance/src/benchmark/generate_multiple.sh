#!/bin/bash

# Specify the output directory
output_dir="../../output"

k="$1"
threads="$2"

echo "Generation,Sorting,Writing" >"vault_$threads"t_"$k.csv"

for n in {1..10}; do
    # Clean the output directory
    rm -rf "${output_dir:?}"/*

    free >/dev/null && sync >/dev/null && sudo sh -c 'echo 3 > /proc/sys/vm/drop_caches' && free >/dev/null
    sudo sync
    sleep 5

    # Capture the output of the program
    ./../../target/release/entrance -k $k -t $threads >>"vault_$threads"t_"$k.csv"
done

mv "vault_$threads"t_"$k.csv" vault_csv/