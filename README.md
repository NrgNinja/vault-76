# Vault 76
## Purpose

This repo contains a new approach, written in Rust, of a tool to further improve Proof-of-Space consensus algorithm for blockchain systems. This was based on the original ([Vault](https://github.com/udig-v/vault)),  which was entirely written in C. The new implementation, Vault-76, will generate records using the BLAKE3 hashing function by taking in a unique & random nonce. Each record contains a 6-byte nonce & a 26-byte hash, which equates to a total record of 32 bytes. Once generated, the hashes will be saved in memory to a DashMap, which is a HashMap that supports concurrency. Then, they are sorted and written to disk to a binary output file. You can then look up the hashes from the binary file by deserializing it. This is a multi-threaded implementation using the data parallelism library, Rayon. Our solution currently has higher throughput and lower latency compared to state-of-the-art [Chia plotters](https://docs.chia.net/plotting-software/).

[Logo](https://drive.google.com/file/d/13utk5G9_SNyEJShodPVpur2Xc5J6A6EU/view?usp=sharing)

## How To Install Rust
*If you are on MacOS or Linux:*

To download Rustup and install Rust, run the following in your terminal, then follow the on-screen instructions.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

*If you are on Windows:*

Follow the instructions [here](https://www.rust-lang.org/tools/install)

For more information, go to [official Rust documentation](https://doc.rust-lang.org/book/ch01-01-installation.html)

## How To Run The Vault
1. Clone the repository to your local machine
```bash
git clone https://github.com/NrgNinja/vault-76.git
```
2. Go through the "entrance" directory
```bash
cd vault-76/entrance
```
*Make sure you have Cargo installed:*

3. Check what version of Cargo you have with:
```bash
cargo --version
```

4. Check that you are able to compile, without actually generating code:
```bash
cargo check --release
```

5. Use this to get all the dependencies right:
```bash
cargo build --release
```
6. To run the vault, use this:
Here are some default settings:
* *output file: `output.bin`*
* *number of threads to use: `1`*
* *number of hashes to generate: `1`* 
* *number of records to print to command line: `0`*
* *sorting functionality: `true` (turned on)*
```bash
cargo run --release
```
Make sure to include `--` after `--release`, if you plan on using more flags.

### Example:
```bash
cargo run --release -- -k 25 -t 8 -p 10
```
*This runs vault operations with `8` threads and generates 2^k records, where k is `25` (so 33,554,432  records). Sorting is on (`true`) by default, and will be written to the output file `output.bin`. Finally, `10` records will be printed to the command line.*

6. To run the program in debug mode, make sure to include the *-b* flag:
```bash
cargo run --release -- -k 25 -t 8 -b
```

7. To see what flags can be customized:
```bash
cargo run --release -- -h
```

8. To clean wipe your build:

*Be sure to remove generated files every once in a while to clean cache and start fresh in case of any issues*
```bash
cargo clean --release
```

## Benchmarking
1. Build a release executable
```bash
cargo build --release
```
2. Go to src directory (if you are in the entrance directory):
```bash
cd src
```
3. Run bash script, specifying parameters
```bash
bash run.sh [num_nonces]
```
The script first cleans cache, then runs release build with specified parameters. 


## Additional Libraries/Dependencies Used 
### BLAKE3
[Cryptographic hash function. Native to Rust.](https://github.com/BLAKE3-team/BLAKE3) 

### Rayon
[Allows for the allocation of multiple threads to perform actions.](https://github.com/rayon-rs/rayon)

### bincode
[Library used for serialization/deserialization of data structures.](https://github.com/bincode-org/bincode)

### BufReader
[Improves performance of reading data. Reads larger chunks of data at once.](https://doc.rust-lang.org/std/io/struct.BufReader.html) 

### Arc
[Enables data sharing among multiple threads.](https://doc.rust-lang.org/std/sync/struct.Arc.html)

### Mutex
[Allows multiple threads to access a shared resource while ensuring that only one thread can access it at a time.](https://doc.rust-lang.org/std/sync/struct.Mutex.html)

### Postcard
[Postcard is a #![no_std] focused serializer and deserializer for Serde..](https://docs.rs/postcard/latest/postcard/)

## Future Work Considerations:
* keep README up to date 
* implement caching layer with NVMe + HDD solutions
* explore lossy/lossless compression techniques

## About the Authors:
* [Varvara Bondarenko](varvara.bondarenko14@gmail.com) 
* [Renato Diaz](https://www.linkedin.com/in/renato-diaz/)
