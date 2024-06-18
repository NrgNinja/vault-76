# Vault 76
## Purpose

This holds the new implementation of the OG vault. The new vault, it being the 76th iteration, will generate hashes according to the BLAKE3 hashing function by taking in a 6-8 byte nonce. Once generated, the hashes will be saved in a Vector (dynamic array). Then, they can be sorted in sequential order, and finally written to a binary output file. You can then lookup the hashes from the binary file. This is a multi-threaded implementation using Rayon in Rust. More details to come.

## How To Install Rust

*If you are on MacOS:*

To download Rustup and install Rust, run the following in your terminal, then follow the on-screen instructions.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## About the Authors:
* [Varvara Bondarenko](varvara.bondarenko14@gmail.com) 
* [Renato Diaz](https://www.linkedin.com/in/renato-diaz/)

## TODO:
*include instructions on:* 
* how to build (ex. all the dependencies needed to run our vault)
* mention BLAKE3, include links to repo, etc.
* how to use (commands to generate, store, view, etc.)
* known bugs
* etc.

## How To Run

First, you have to make sure that you are in the correct directory:
```bash
cd entrance
```

Then, you gotta make sure you have Cargo installed:

Check that you are able to compile, without actually compiling:
```bash
cargo check
```

Make sure to use this to get all the dependencies right:
```bash
cargo build --release
```
To run from `entrance` directory:
```bash
cargo run --release -n 33554432 -t 16
```
*where:*
* -n represents, num of nonces
* -f represents, output filename // not necessary, default is output.bin
* -t represents, num of threads

*Be sure to remove generated files every once in a while or else it might start breaking your computer:*
```bash
cargo clean --release
```
