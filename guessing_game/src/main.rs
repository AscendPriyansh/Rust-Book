use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your number : ");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to take user input");

        let guess: u32 = guess.trim().parse().expect("Failed to parse!");

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You Won!");
                break;
            }
            Ordering::Greater => println!("Too Big!"),
            Ordering::Less => println!("Too Small"),
        };
    }
}
