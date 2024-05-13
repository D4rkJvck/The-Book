use rand::Rng;

pub fn add_random_digit(x: i32) -> i32 {
    let plus = rand::thread_rng().gen_range(1..=9);
    x + plus
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_random_digit(2) > 2;
        assert_eq!(result, true);
    }
}
