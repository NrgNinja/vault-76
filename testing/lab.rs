// this allows for unused code to bypass warnings
#![allow(dead_code)]

// this struct is as close we'll get to creating an object
#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

// using the standard input/ouput library
use std::io;
use std::env;

// // functions can be outside and below main if you want (just like python)
// // prints the type of the variable that you are looking for
// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

fn main() {
    // // formatting strings
    // println!("Hello Mr. {lastname}, how are you doing today? I hope your trip was okay, {firstname}.", firstname = "Renato", lastname = "Diaz");
    // println!("You are {:b} years old, correct?", 23);
    // println!("Let's play with this number, {num:0<5}", num = 1);
    // println!("What happens if I don't have the right # of args? {0}, {1}", "Renato", 23);
    // println!("What about this one? {:?}", "Renato");
    // let pi = 3.141592;
    // println!("Pi should be approximately: {:.2}", pi);

    // creating structs are like creating objects
    // let name = String::from("Renato");
    // let age = 23;
    // let renato = Person {name, age};
    // println!("{:#?}", renato);

    // // find out the type of a variable
    // let fullname: bool = true;
    // println!("{}", fullname);
    // print_type_of(&fullname);

    // // 'mut' makes the variable mutable, but you cannot change the type unless you add 'let'
    // let mut name = "renato";
    // let name = 23;
    // println!("{}", name);

    // // creating tuples and printing them
    // // note that you can't create tuples of more than 12 elements for some reason
    // let name_tuple = ("renato", "varvara", "josh", "tom");
    // let num_tuple = (1, 2, 3, 4, 5);
    // println!("{:?}", name_tuple);
    // println!("{:?}", num_tuple);

    // // indexing an array
    // let arr = [1, 2, 3, 4, 5];
    // println!("{}", arr[1]);

    // println!("Guess the number!");
    // println!("Please input your guess.");
    // let mut guess = String::new();
    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read line");
    // println!("You guessed: {guess}");

    let args: Vec<String> = env::args().collect();
    let num1 = &args[1];
    let num2 = &args[2];
    println!("here is the first num: {}, and here is the second num: {}", num1, num2);
}
