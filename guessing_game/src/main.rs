use rand::Rng;
use std::{cmp::Ordering, io};

// struct Guess {
//     value: i32,
// }

// impl Guess {
//     fn new(value: i32) -> Guess {
//         if value < 1 || value > 100 {
//             panic!("Guess value must be between 1 and 100, got {:?}.", value);
//         }

//         Guess { value }
//     }

//     fn value(&self) -> i32 {
//         self.value
//     }
// }

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin() //---                               ---> Could also use std::io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100!");
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
