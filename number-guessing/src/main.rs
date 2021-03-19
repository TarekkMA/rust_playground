use std::io;
use rand::Rng;
use std::cmp::Ordering::{Equal, Greater, Less};

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut counter = 0;

    println!("Welcome to the guessing game!\nEnter quit to exit\n");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user's guess");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if guess.trim().eq_ignore_ascii_case("quit") {
                    break;
                } else {
                    println!("'{}' is not a valid number", guess);
                    continue;
                }
            }
        };
        counter += 1;
        match guess.cmp(&secret_number) {
            Equal => {
                println!("You Won after {} guesses!!", &counter);
                break;
            }
            Less => println!("Guess higher"),
            Greater => println!("Guess lower"),
        };
    }
}
