use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    /*
    In rust variables are immutable by default
    */
    let _apples = 5;
    let mut bananas = 5; // mutable 

    

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
