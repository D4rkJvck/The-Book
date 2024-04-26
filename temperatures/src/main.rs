use std::io;

fn main() {
    loop {
        println!("Temperature (Celsius): ???");

        let mut celsius = String::new();

        io::stdin()
            .read_line(&mut celsius)
            .expect("Fail to read line");

        let celsius: i32 = match celsius.trim().parse() {
            Ok(temp_c) => temp_c,
            Err(_) => {
                println!("Invalid Number");
                continue;
            }
        };

        let fahrenheit = (celsius * 9 / 5) + 32;

        println!("Temperature (Fahrenheit): {fahrenheit}");
        println!("Would you like to try again? (y/n)");

        let mut answer = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("Fail to read line");

        if answer.trim() == "y" {
            continue;
        } else if answer.trim() == "n" {
            println!("Goodbye!");
        } else {
            println!("I'll take it as a NO")
        }

        break;
    }
}
