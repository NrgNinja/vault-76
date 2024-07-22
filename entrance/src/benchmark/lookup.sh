#!/bin/bash

k=$1
hash_len=$2
total_duration=0

# generate file
output_dir="../../output"
rm -rf "${output_dir:?}"/*
./../../target/release/entrance -k $k -t 16 -f output.bin

# add headings
echo "HashPrefix,LookupTime(ms),IsExist" >lookup_$k"_"$hash_len.csv

# run lookup
for i in {1..1000}; do
    # clean cache
    free >/dev/null && sync >/dev/null && sudo sh -c 'echo 3 > /proc/sys/vm/drop_caches' && free >/dev/null
    hash=$(python3 get_hash.py $i $hash_len)
    ./../../target/release/entrance -l $hash -f output.bin >>lookup_$k"_"$hash_len.csv
done

mv lookup_$k"_"$hash_len.csv lookup_csv/