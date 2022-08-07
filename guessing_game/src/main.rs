use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number?");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your guess:");
        let guess = make_guess();

        println!("You guessed: {guess}");
        let guess_equal_to_secret = compare_guess_to_secret_number(guess, secret_number);

        if guess_equal_to_secret {
            break;
        }
    }
}

fn make_guess() -> u32 {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line!");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    return guess;
}

fn compare_guess_to_secret_number(guess: u32, secret_number: u32) -> bool {
    match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("Too small!");
            false
        }
        Ordering::Greater => {
            println!("Too big!");
            false
        }
        Ordering::Equal => {
            println!("You win!");
            true
        }
    }
}
