#!/bin/bash

k="$1"
threads="$2"
cd ../..

# Specify the output directory
output_dir="output"

free >/dev/null && sync >/dev/null && sudo sh -c 'echo 3 > /proc/sys/vm/drop_caches' && free >/dev/null
sleep 3

# Clean the output directory
echo "Cleaning the output directory..."
rm -rf "${output_dir:?}"/*

sar -u 1 > stats/cpu/cpu-stats_$k$threads.txt &
sar -b 1 > stats/io/io-stats_$k$threads.txt &
sar -r 1 > stats/memory/memory-stats_$k$threads.txt &

./target/release/entrance -k $k -t $threads -p 10
pkill sar

file_size=$(du -hs $output_dir)
echo "The total size of all files is $file_size"
