free > /dev/null && sync > /dev/null && sudo sh -c 'echo 3 > /proc/sys/vm/drop_caches' && free > /dev/null
sleep 5
k="$1"
threads="$2"
print="$3"
cd .. 
./target/release/entrance -k $k -t $threads -s true -f output.bin -p $print
# cargo run --release -- -k $k -t 16