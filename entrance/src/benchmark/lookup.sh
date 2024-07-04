#!/bin/bash

free >/dev/null && sync >/dev/null && sudo sh -c 'echo 3 > /proc/sys/vm/drop_caches' && free >/dev/null
sleep 5

hash=$1

cd ../..
./target/release/entrance -h $hash
