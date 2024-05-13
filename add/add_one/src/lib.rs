pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_one_to_three() {
        let result = add_one(3);
        assert_eq!(result, 4);
    }
}
