use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Please guess a number");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read"); // read the number as string
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your number is less, try again."),
            Ordering::Greater => println!("Your number is greated, try again."),
            Ordering::Equal => {
                println!("You guessed correct");
                break;
            }
        }
    }
}
