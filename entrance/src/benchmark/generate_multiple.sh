#!/bin/bash

# Specify the output directory
output_dir="../../output"

k="$1"
threads="$2"

# for k in {29..30}; do
    # for threads in 2 4 8 16; do
        echo "Generation,Sorting,Writing" >vault_csv/vault_$threads"t_"$k.csv

        for n in {1..5}; do
            # Clean the output directory
            rm -rf "${output_dir:?}"/*
            sync

            free >/dev/null && sync >/dev/null && sudo sh -c 'echo 3 > /proc/sys/vm/drop_caches' && free >/dev/null
            sync
            sleep 5

            # Capture the output of the program
            ./../../target/release/entrance -k $k -t $threads >>vault_csv/vault_$threads"t_"$k.csv
        done
    # done
# done
