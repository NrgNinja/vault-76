// this file will hold the main driver of our vault codebase

// we can separate different components of the toolkit into different files:
// ex: generation of hashes    --> hash-generator.rs
//     organize the hashes     --> hash-organizer.rs
//     sort the hashes         --> hash-sorter.rs
//     store the hashes        --> store-file.rs

fn main() {
    let name = "renato";
    println!("The vault will go here! {}" , name);
}