# Vault 76
## Purpose

This holds the new implementation, written in Rust, of the OG vault which was entirely written in C. The new vault, it being the 76th iteration, will generate records using the BLAKE3 hashing function by taking in a unique nonce. Each record contains a 6-byte nonce & a 26-byte hash, which equates to a total record of 32 bytes. Once generated, the hashes will be saved in memory to a vector (dynamic array). Then, they can be sorted in sequential order, and finally written to disk on a binary output file. You can then look up the hashes from the binary file by deserializing it. This is a multi-threaded implementation using the data parallelism library, Rayon. More details to be added.

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
6. To run the vault with default settings, use this:
* *output file: `output.bin`*
* *number of threads to use: 4*
* *number of records to generate: 10* 
* *number of records to print: 10*
* *sorting functionality: true (turned on)*
```bash
cargo run --release
```
Make sure to include `--` after `--release`, if you plan on using more flags.

### Example:
```bash
cargo run --release -- -n 33554432 -t 16
```
*This runs the vault with 16 threads and generates 33554432 records, sorting is on by default, and will be written to output.bin.*

6. To see what flags can be customized:
```bash
cargo run --release -- -h
```

7. To clean wipe your build (in case of any issues):

*Be sure to remove generated files every once in a while or else it might start breaking your computer:*
```bash
cargo clean --release
```

## Additional Libraries/Dependencies Used 
### BLAKE3
[Cryptographic hash function. Native to Rust.](https://github.com/BLAKE3-team/BLAKE3) 

### Rayon
[Allows for the allocation of multiple threads to perform actions.](https://github.com/rayon-rs/rayon)

### bincode
[Library used for serialization/deserialization of data structures.](https://github.com/bincode-org/bincode)

### BufReader
[Improves performance of reading data. Reads larger chunks of data at once.](https://doc.rust-lang.org/std/io/struct.BufReader.html) 

## Known Bugs
* It's too slow rn, we're working on making it faster lol. There might be an issue with how we're generating & storing hashes.

## TODO:
*include instructions on:* 
* how to build (ex. all the dependencies needed to run our vault) 
* mention BLAKE3, include links to repo, etc.
* how to use (commands to generate, store, view, etc.)
* known bugs
* etc.

## About the Authors:
* [Varvara Bondarenko](varvara.bondarenko14@gmail.com) 
* [Renato Diaz](https://www.linkedin.com/in/renato-diaz/)
