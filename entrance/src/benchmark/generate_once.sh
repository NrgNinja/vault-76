#!/bin/bash

k="$1"
threads="$2"
memory="$3"
hash_size="$4"

# Specify the output directory
output_dir="../../output"
# Clean the output directory
echo "Cleaning the output directory..."
rm -rf "${output_dir:?}"/*
sudo sync

free >/dev/null && sync >/dev/null && sudo sh -c 'echo 3 > /proc/sys/vm/drop_caches' && free >/dev/null
sudo sync

# sar -u 1 >stats/cpu/cpu-stats_$k$threads.txt &
# sar -b 1 >stats/io/io-stats_$k$threads.txt &
# sar -r 1 >../../stats/memory/memory-stats_$k$threads.txt &
# sleep 5

./../../target/release/entrance -k $k -t $threads -s -m $memory -d -p 100 -h $hash_size
# 17179869184
# 2147483648
# dd if=/dev/urandom of=newfile bs=1M count=1024
# shred -s 1000000000 - >my-file
sleep 5

pkill sar

file_size=$(du -hs $output_dir)
echo "The total size of all files is $file_size"
