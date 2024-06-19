# Vault 76
## Purpose

This holds the new implementation of the OG vault. The new vault, it being the 76th iteration, will generate hashes according to the BLAKE3 hashing function by taking in a unique 6 byte nonce. Once generated, the hashes will be saved in a Vector (dynamic array). Then, they can be sorted in sequential order, and finally written to a binary output file. You can then lookup the hashes from the binary file. This is a multi-threaded implementation using Rayon in Rust. More details to come.

## How To Install Rust
*If you are on MacOS or Linux:*

To download Rustup and install Rust, run the following in your terminal, then follow the on-screen instructions.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

*If you are on Windows:*

Follow the instructions [here](https://www.rust-lang.org/tools/install)

For more information, go to [official Rust documentation](https://doc.rust-lang.org/book/ch01-01-installation.html)

## How To Run 
1. Clone the repository to your local machine
```bash
  git clone https://github.com/NrgNinja/vault-76.git
```
2. Go into your project directory
```bash
  cd vault-76/entrance
```
*Also make sure you have Cargo installed:*

3. Check that you are able to compile, without actually compiling:
```bash
cargo check
```

4. Make sure to use this to get all the dependencies right:
```bash
cargo build --release
```
5. To run the program with default settings:
*output file: `output.bin`*
*number of threads to use: 4*
*number of records to generate: 10* 
*number of records to print: 10*
*sorting functionality: true (turned on)*
```bash
  cargo run --release
```
*Example:*
```bash
cargo run --release -- -n 33554432 -t 16
```
*This runs the vault with 16 threads and generates 33554432 records, sorting is on by default, and will be written to output.bin.*

6. To see what flags can be customized:
```bash
  cargo run --release -- -h
```

7. To clean wipe your build (for performance):
*Be sure to remove generated files every once in a while or else it might start breaking your computer:*
```bash
cargo clean --release
```

## Additional Libraries Used 
### BLAKE3
Cryptographic hash function. 

### Rayon
Allows for the allocation of multiple threads to perform actions.

### bincode
Library used for serialization/deserialization of data structures. 

### BufReader
Improves performance of reading data. Reads larger chunks of data at once. 

## Known Bugs
* too slow rn, we're working on making it faster lol

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