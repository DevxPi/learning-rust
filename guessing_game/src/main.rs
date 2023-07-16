use std::io; // To obtain user input and then print the result as output
use rand::Rng; // rng stands for `random number generator`

fn main(){
    println!("Guess the number!");

    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new(); // create a variable to store the user input.  variables are immutable by default, meaning once we give the variable a value, the value won’t change

    // To make a variable mutable, we add mut before the variable name:

    io::stdin()
        .read_line(&mut guess) // & -> reference
        .expect("Failed to read line");

    println!("You gueesed: {guess}");
}