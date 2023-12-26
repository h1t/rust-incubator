use std::{cmp::Ordering, io};
use step_3_1::{RandomNumberGenerator, State};

fn main() {
    println!("Guess the number!");

    let state = State::new(RandomNumberGenerator {});

    loop {
        println!("Please input your guess.");

        let number = match get_guess_number() {
            Some(n) => n,
            _ => continue,
        };

        println!("You guessed: {}", number);

        match state.guess(number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn get_guess_number() -> Option<u32> {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess.trim().parse().ok()
}
