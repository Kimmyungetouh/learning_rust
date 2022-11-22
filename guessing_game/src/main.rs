use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number !");
    println!("Choosing a secret number ...");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts : u32 = 0;
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        attempts += 1;
        // println!("You guessed : {}!\n", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("\n *** TOO SMALL ! *** \n"),
            Ordering::Greater => println!("\n*** TOO BIG ! *** \n"),
            Ordering::Equal =>{
                println!(
                    "Well done ! The secret number is {secret_number} You guessed in {attempts} attempts !");
                break;
            }

        }
    }
}
