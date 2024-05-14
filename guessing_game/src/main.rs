//! # Guessing Game
//! A fun game where you have 5 chances to guess what number the computer has chosen.

use d4rk_guessing_game::{cooldown, prompt, timer};
use rand::Rng;
use std::{cmp::Ordering, process::exit};

/// Generates a random number that will be kept secret till the end.
fn main() {
    cooldown("\nGuess the number between 1 and 100 !", 1);

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut count = 5;

    while count > 0 {
        timer(&mut count);

        let guess = match prompt() {
            Some(guess) => guess,
            None => continue,
        };

        // Compare the generated number with the user input for equality
        match guess.value().cmp(&secret_number) {
            Ordering::Less => cooldown("TOO small !", 1),
            Ordering::Greater => cooldown("too BIG !", 1),
            Ordering::Equal => {
                println!("\nYOU WIN !");
                exit(0);
            }
        }
    }

    cooldown(&format!("\nThe number to guess was: {secret_number}"), 1);
    println!("YOU LOSE !")
}
