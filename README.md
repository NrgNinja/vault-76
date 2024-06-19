# Vault 76
## Purpose

This will hold the new implementation of the vault. The new vault, it being the 76th iteration, will generate hashes according to the BLAKE3 hashing function by taking in a 6-8 byte nonce. Once generated, the hashes will be saved in a *[insert data structure to use here]*. Then, they can be sorted in sequential order, and finally written to a file. This implementation is in Rust. More details to come.

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
  1. Install Rust and Cargo [here](https://www.rust-lang.org/tools/install) and [here](https://doc.rust-lang.org/book/ch01-01-installation.html)
  2. Check if tools are installed ```cargo -V``` *You may need to close all of your terminals first*
  3. Clone repository to your machine. ```git clone https://github.com/NrgNinja/vault-76.git```
  4. Go into project directory -> entrance.
  5. ```cargo run``` 
* mention BLAKE3, include links to repo, etc.
    bincode - library used for serialization/deserialization of data structures. 
    BufReader - improves performance of reading data. Reads larger chunks of data at once. 
* how to use (commands to generate, store, view, etc.)
* known bugs
* etc.
