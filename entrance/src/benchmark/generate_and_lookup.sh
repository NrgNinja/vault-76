#!bin/bash

# Generate a file with k = 38 and 16 threads
/bin/bash generate_once.sh 29 16
echo

max_time=0
min_time=99999999
total_avg_time=0
num_runs=1000

for n in {1..1000}; do
    # Generate a random length between 1 and 5
    length=$((RANDOM % 5 + 1))
    # Generate a random hex hash of the specified length
    hash=$(openssl rand -hex $length)

    echo "------------------------------($n/$num_runs) Looking up hash $hash...----------------------------------"
    output=$(/bin/bash lookup.sh "$hash")

    # Extract the average time from the output
    average_time=$(echo "$output" | awk '/Average time/ {print $3}')
    total_avg_time=$(echo "$total_avg_time + $average_time" | bc)
    echo "$output"

    # Update max_time and min_time
    if (($(echo "$average_time > $max_time" | bc -l))); then
        max_time=$average_time
    fi

    if (($(echo "$average_time < $min_time" | bc -l))); then
        min_time=$average_time
    fi

    echo
done

# Calculate the average time
average_time=$(echo "scale=3; $total_avg_time / $num_runs" | bc)

# Print the maximum and minimum average times
echo "Total average time: $average_time ms"
echo "Maximum average time: $max_time ms"
echo "Minimum average time: $min_time ms"
