use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use webbrowser;

fn main() {
    webbrowser::open("https://www.naver.com").unwrap();
}

pub fn add_one(original: u32) -> u32 {
    return original + 1;
}

pub fn make_guide_text() {
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

        match guess.trim().parse() {
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

pub fn test_python() -> PyResult<()> {
    Python::with_gil(|py| {
        let sys = py.import("sys")?;
        let version: String = sys.getattr("version")?.extract()?;

        let locals = [("os", py.import("os")?)].into_py_dict(py);
        let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
        let user: String = py.eval(code, None, Some(&locals))?.extract()?;

        println!("Hello {}, I'm Python {}", user, version);
        Ok(())
    })
}
