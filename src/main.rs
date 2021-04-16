use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("Input any number: ");
    let mut guess = String::new();  // Creating mutable variable
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!!");
    println!("Secret number was: {}", secret_number);
    println!("You guessed: {}", guess);
}
