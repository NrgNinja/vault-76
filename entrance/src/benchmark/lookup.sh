#!/bin/bash

hash=$1
total_duration=0
total_time=0
num_runs=5

cd ../..

for n in {1..5}; do
    echo "--------------------------------------Run $n------------------------------------------------"
    free >/dev/null && sync >/dev/null && sudo sh -c 'echo 3 > /proc/sys/vm/drop_caches' && free >/dev/null
    sleep 5

    output=$(./target/release/entrance -x $hash)
    echo "$output"

    total_duration=$(echo "$output" | grep -oP 'Looking up.*hash took \K[0-9]+\.[0-9]+')
    total_time=$(echo "$total_time + $total_duration" | bc)

    echo
done

average_time=$(echo "scale=3; $total_time / $num_runs" | bc)
echo "Average time: $average_time"
