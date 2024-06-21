free > /dev/null && sync > /dev/null && sudo sh -c 'echo 3 > /proc/sys/vm/drop_caches' && free > /dev/null
sleep 5
nonces="$1"
cargo run --release -- -n $nonces -t 8