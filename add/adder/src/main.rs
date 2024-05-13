use add_one::add_one;
use add_random::add_random_digit;
use add_two::add_two;

fn main() {
    let num = 10;
    let rand_result = add_random_digit(num);
    println!("{num} plus one is {:?}", add_one(num));
    println!("{num} plus two is {:?}", add_two(num));
    println!(
        "The random number added to {num} to give {:?} is {:?}",
        rand_result,
        rand_result - num
    );
}
