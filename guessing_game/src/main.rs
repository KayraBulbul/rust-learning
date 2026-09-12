use rand::RngExt;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::rng().random_range(1..=100);
    // println!("The secret number is: {}", secret_number);
    let mut guess_count = 0;

    loop {
        println!("Input your guess:");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read_line");

        let guess: u32 = guess.trim().parse().expect("Please type a valid number!");
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!!");
                guess_count += 1;
            }
            Ordering::Greater => {
                println!("Too big!!");
                guess_count += 1;
            }
            Ordering::Equal => {
                println!("You guessed it correctly!");
                println!("It took you {} guesses.", guess_count);
                break;
            }
        }
    }
}
