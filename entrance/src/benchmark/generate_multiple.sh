#!/bin/bash

# Specify the output directory
output_dir="output"

k="$1"
threads="$2"
cd ../..

generating_time=0
sorting_time=0
writing_time=0
total_time=0
num_runs=10

for n in {1..10}; do
    # Clean the output directory
    echo "Cleaning the output directory..."
    rm -rf "${output_dir:?}"/*

    free >/dev/null && sync >/dev/null && sudo sh -c 'echo 3 > /proc/sys/vm/drop_caches' && free >/dev/null
    sleep 5

    # Start the sar command to monitor CPU and IO stats
    # sar -u 1 >stats/cpu/cpu-stats_$n.txt &
    # sar -b 1 >stats/io/io-stats_$n.txt &
    # sar -r 1 >stats/memory/memory-stats_$n.txt &

    echo ----------------------------------------Run $n---------------------------------------------

    # Capture the output of the program
    output=$(./target/release/entrance -k $k -t $threads)

    sleep 5
    
    # Stop the sar command
    # pkill sar
    echo "$output"

    # Extract the generating hashes time from the output and add it to the total time for 3 runs
    generating_duration=$(echo "$output" | grep -oP 'Generating.*hashes took \K[0-9]+\.[0-9]+')
    generating_time=$(echo "$generating_time + $generating_duration" | bc)

    # Extract the sorting hashes time from the output
    sorting_duration=$(echo "$output" | grep -oP 'Sorting hashes took \K[0-9]+\.[0-9]+')
    sorting_time=$(echo "$sorting_time + $sorting_duration" | bc)

    # Extract the sorting hashes time from the output
    writing_duration=$(echo "$output" | grep -oP 'Writing hashes to disk took \K[0-9]+\.[0-9]+')
    writing_time=$(echo "$writing_time + $writing_duration" | bc)

    # Extract the duration from the output
    total_duration=$(echo "$output" | grep -oP 'Generated.*records in \K[0-9]+\.[0-9]+')
    # Add the duration to the total time
    total_time=$(echo "$total_time + $total_duration" | bc)

    echo
done

echo ----------------------------------------Results---------------------------------------------

# Calculate the average time
avg_generating_time=$(echo "scale=3; $generating_time / $num_runs" | bc)
avg_sorting_time=$(echo "scale=3; $sorting_time / $num_runs" | bc)
avg_writing_time=$(echo "scale=3; $writing_time / $num_runs" | bc)
average_time=$(echo "scale=3; $total_time / $num_runs" | bc)

echo "Average generating time: $avg_generating_time"
echo "Average sorting time: $avg_sorting_time"
echo "Average writing to disk time: $avg_writing_time"
echo "Average time: $average_time"
