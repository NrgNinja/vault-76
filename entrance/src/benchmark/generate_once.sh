#!/bin/bash

# Specify the output directory
output_dir="../output"

# Clean the output directory
echo "Cleaning the output directory..."
rm -rf "${output_dir:?}"/*

free >/dev/null && sync >/dev/null && sudo sh -c 'echo 3 > /proc/sys/vm/drop_caches' && free >/dev/null
sleep 5

k="$1"
threads="$2"
cd ../..

./target/release/entrance -k $k -t $threads -p 10