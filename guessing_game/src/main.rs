use rand::Rng;
use std::{cmp::Ordering, io};

struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {:?}.",
                value,
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {:?}.",
                value,
            );
        }

        Guess { value }
    }

    fn value(&self) -> i32 {
        self.value
    }
}

///////////////////////////////////////////////////////////////////////////////////////

fn main() {
    println!("Guess the number between 1 and 100!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Please input your guess.");
        
        let mut guess = String::new();
        
        io::stdin()
        .read_line(&mut guess)
        .expect("Fail to read line");
    
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess = Guess::new(guess);

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "greater than or equal to 1")]
    fn less_than_1() {
        Guess::new(0);
    }
}