#!/bin/bash

hash=$1
total_duration=0
total_time=0
num_runs=5

# Generate a file with k = 38 and 16 threads
/bin/bash generate_once.sh 29 16
echo

cd ../..

echo "--------------------------------------Run $n------------------------------------------------"
free >/dev/null && sync >/dev/null && sudo sh -c 'echo 3 > /proc/sys/vm/drop_caches' && free >/dev/null
sleep 5

# sar -u 1 >cpu-stats_$n.txt &
# sar -b 1 >io-stats_$n.txt &

output=$(./target/release/entrance -x $hash)

# pkill sar
echo "$output"

total_duration=$(echo "$output" | grep -oP 'Looking up.*hash took \K[0-9]+\.[0-9]+')
echo "$total_duration"

echo
