use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    let mut guess: String;
    let mut guess_number: u32;

    loop {
        guess = String::new();

        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        guess = guess.trim().to_string();

        match guess.parse() {
            Ok(number) => guess_number = number,
            Err(_error) => {
                guess = "0".to_string();
                guess_number = 0;
            }
        };

        println!("You guessed: {guess}");

        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
