// To get user input and print output we need io library in scope that comes from standard library std
// The prelude module contains the most commonly used parts of the rand crate
use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    // start..=end
    let secret_number = rand::rng().random_range(1..=100);

    // println!("The secret number is: {secret_number}");
    loop {
        println!("Please input your guess.");

        /*
        A variable to store users input
        let statement creates a variable -> let apples = 5;
        Variables are immutable by default in Rust
        let apples = 5; //immutable
        let mut bananas = 5; //mutable
        */
        let mut guess = String::new(); //create a mutable instance bounded to a new, empty instance of a string

        //  calling stdin from io to handle user input - std::io::stdin
        // read_line(&mut guess) - calls the read_line method on the stdin
        // The & indicates that this argument is a reference. references are immutable by default as well - that's why you need &mut guess
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // expect("Please type a number!");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
