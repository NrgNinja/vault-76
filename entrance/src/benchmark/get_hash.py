import random
import sys

random.seed(sys.argv[1])
hash_len = int(sys.argv[2]) // 2
hash_bits = random.getrandbits(hash_len * 8)  # length in bytes * 8 bits per byte
hash = f'{hash_bits:0{hash_len * 2}x}'  # convert to hex and pad with zeros if needed

print(hash)