use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;

fn main() {
    println!("Guess the {}!", "number".cyan());

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("{} input your {}.", "Please".yellow(), "guess".bright_blue());

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too {}!", "small".red()),
            Ordering::Greater => println!("Too {}!", "big".red()),
            Ordering::Equal => {
                println!("You {}", "win!".bright_blue());
                break;
            }
        }
    }
}
