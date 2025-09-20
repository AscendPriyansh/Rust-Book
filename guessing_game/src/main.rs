use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");
    
    let mut count = 0;

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your number : ");
        count+=1;

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to take user input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter valid input");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You Won!");
                println!("Tries : {count}");
                break;
            }
            Ordering::Greater => println!("Too Big!"),
            Ordering::Less => println!("Too Small"),
        };
    }
}
