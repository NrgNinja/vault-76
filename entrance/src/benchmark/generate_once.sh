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

./target/release/entrance -k $k -t $threads -p 10
