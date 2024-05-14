use std::{io, thread::sleep, time::Duration};

/// Will hold the Value of the player input
pub struct Guess {
    value: i32,
}

/// Creates a new instance of a Guess struct
/// that hold the given value if valid.
impl Guess {
    fn new(value: i32) -> Result<Self, String> {
        match value {
            v if v < 1 => Err(format!("{v} is less than 1!")),
            v if v > 100 => Err(format!("{v} is greater than 100!")),
            _ => Ok(Self { value }),
        }
    }

    /// Gives access to the value of the Guess
    pub fn value(&self) -> i32 {
        self.value
    }
}

////////////////////////////////////////////////////////////////////////////////////////

/// Prompt the user to input a number of his choice between 1 and 100.
/// Then try to Convert the input into a Integer.
/// If it's not an integer, the player loses a chance anyway.
/// If it is a new instance of Guess is created...
/// assuming the integer is between 1 and 100.
pub fn prompt() -> Option<Guess> {
    println!("\nInput your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("\nFail to read line");

    cooldown("Checking...", 3);

    let guess = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            cooldown(&format!("{:?} is not a Number!", guess.trim()), 1);
            return None;
        }
    };

    match Guess::new(guess) {
        Ok(g) => Some(g),
        Err(msg) => {
            cooldown(&format!("{msg}"), 1);
            None
        }
    }
}

//---------------------------------------

/// Slow Game process down...
pub fn cooldown(msg: &str, dur: u64) {
    println!("{msg}");
    sleep(Duration::from_secs(dur));
}

//-------------------------------------------------------------------

/// Lets the player know the remaining chances.
/// Counts down the given number of chance.
pub fn timer(count: &mut u32) {
    match count {
        1 => cooldown("\nLAST CHANCE !", 1),
        5 => cooldown("\nYou have 5 chances !", 1),
        _ => cooldown(&format!("\n{count} chances left !"), 1),
    }

    *count -= 1;
}
