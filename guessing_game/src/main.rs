use std::io;
use rand::R
fn main() {
    println!("Guess the number !");
    println!("Please in put yout guess.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed : {}!\n", guess);
}
