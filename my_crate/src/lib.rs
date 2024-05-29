//! # My Crate
//!
//! 'my_crate' is a collection of utilitiesto make performing certain
//! calculations more convenient.

/// Increments a given number by one
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```

//////////////////////////////////////////////////////////////////////////////

pub fn add_one(x: i32) -> i32 {
    x + 1
}

//////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_one_to_5() {
        assert_eq!(6, add_one(5))
    }
}
