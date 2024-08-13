#!/bin/bash

# Specify the output directory
output_dir="../../output"

k="$1"
threads="$2"
memory="$3"

echo "Gen&Flush,Sort,Sync" >"vault_csv/vault76_eightsocket_$k"_"$threads"t".csv"

for n in {1..5}; do
for k in {25..30}; do
    for threads in {1..16}; do
        for memory in {1..8}; do
            # Clean the output directory
            rm -rf "${output_dir:?}"/*

            free >/dev/null && sync >/dev/null && sudo sh -c 'echo 3 > /proc/sys/vm/drop_caches' && free >/dev/null
            sudo sync
            sleep 1

            # Capture the output of the program
            ./../../target/release/entrance -k $k -t $threads -m $memory >>"vault_csv/vault_$k"_"$threads"t".csv"
        done
    done
done

for n in {1..5}; do
    # Clean the output directory
    rm -rf "${output_dir:?}"/*

    free >/dev/null && sync >/dev/null && sudo sh -c 'echo 3 > /proc/sys/vm/drop_caches' && free >/dev/null
    sudo sync
    sleep 1

    # Capture the output of the program

    ./../../target/release/entrance -k $k -t $threads -m $memory >>"vault_csv/vault76_eightsocket_$k"_"$threads"t".csv"
done
