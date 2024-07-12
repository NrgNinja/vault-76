#!/bin/bash

hash=$1
echo "Hash passed to script: $hash"
total_duration=0
cd ../..

echo "Clearing cache..."
free >/dev/null && sync >/dev/null && sudo sh -c 'echo 3 > /proc/sys/vm/drop_caches' && free >/dev/null
sleep 5

echo "Running the command..."
output=$(./target/release/entrance -x $hash)

# pkill sar
echo "$output"

total_duration=$(echo "$output" | grep -oP 'Looking up.*hash took \K[0-9]+\.[0-9]+')
echo "Total duration: $total_duration"

echo
