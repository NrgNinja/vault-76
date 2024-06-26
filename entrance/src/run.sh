free > /dev/null && sync > /dev/null && sudo sh -c 'echo 3 > /proc/sys/vm/drop_caches' && free > /dev/null
sleep 5
k="$1"
cd .. 
./target/release/entrance -k $k -t 16 -s false -f output.bin
# cargo run --release -- -k $k -t 16