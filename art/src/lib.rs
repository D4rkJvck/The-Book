//! # Art
//!
//! A library for modeling artisitic concepts

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// Primary Colors according to the RYB color model
    #[derive(PartialEq)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// Secondary Colors accordint to the RYB color model
    #[derive(PartialEq)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines 2 Primary Colors in equal amounts
    /// to create a Secondary Color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> Option<SecondaryColor> {
        if c1 == c2 {
            None
        } else {
            Some(SecondaryColor::Green)
        }
    }
}
