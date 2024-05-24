use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Number Guessing Game");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    println!("Secret Number: {secret_number}");

    loop {
        println!("Please Input Guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Failed to read line");

        println!("You Guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Guess Too Small"),
            Ordering::Greater => println!("Guess Too Big"),
            Ordering::Equal => {
                println!(" You Win!!!");
                break;
            }
        }
    }
}
