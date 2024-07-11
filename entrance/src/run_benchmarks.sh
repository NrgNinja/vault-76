#!/bin/bash

# Path to the output directory
output_dir="output"

# Parameters from command line
k="$1"
threads="$2"
cd ..

# Initialize times
generating_time=0
writing_time=0
total_time=0
num_runs=6  # Number of runs for averaging

for n in {1..6}; do
    # Clear output directory and system caches
    echo "Cleaning the output directory..."
    rm -rf "${output_dir:?}"/*
    free >/dev/null && sync >/dev/null && sudo sh -c 'echo 3 > /proc/sys/vm/drop_caches' && free >/dev/null
    sleep 3

    sar -u 1 > stats/cpu-stats_$n.txt &
    sar -b 1 > stats/io-stats_$n.txt &
    sar -r 1 > stats/mem-stats_$n.txt &

    echo "----------------------------------------Run $n---------------------------------------------"

    # Run the program and capture the output
    output=$(./target/release/entrance -k $k -t $threads -f output.bin)

    # Kill the sar process
    pkill sar
    echo "$output"

    # Extract and accumulate the time taken for generating and storing hashes
    generating_duration=$(echo "$output" | grep -oP 'Hash generation & storing into DashMap took \K[0-9]+\.[0-9]+')
    generating_time=$(echo "$generating_time + $generating_duration" | bc)

    # Extract and accumulate the time taken for writing hashes to disk
    writing_duration=$(echo "$output" | grep -oP 'Writing hashes to disk took about \K[0-9]+\.[0-9]+')
    writing_time=$(echo "$writing_time + $writing_duration" | bc)

    # Calculate total duration
    total_duration=$(echo "$output" | grep -oP 'Time taken for .* using.*threads: \K[0-9]+\.[0-9]+')
    total_time=$(echo "$total_time + $total_duration" | bc)

    echo
done


echo "----------------------------------------Results---------------------------------------------"

# Calculate and display the average times
avg_generating_time=$(echo "scale=3; $generating_time / $num_runs" | bc)
avg_writing_time=$(echo "scale=3; $writing_time / $num_runs" | bc)
average_time=$(echo "scale=3; $total_time / $num_runs" | bc)

echo "Average generating time: $avg_generating_time seconds"
echo "Average writing to disk time: $avg_writing_time seconds"
echo "Average total time: $average_time seconds"