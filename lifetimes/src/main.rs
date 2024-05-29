#![allow(unused)]

use std::fmt::Display;

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn annoncement_and_return_part(&self, annoncement: &str) -> &str {
        println!("Attention please: {}", annoncement);
        self.part
    }
}

/////////////////////////////////////////////////////////////////////////////////

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a .");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    let s1 = String::from("hello");
    let s2 = "world";

    match longest(s1.as_str(), s2, "Longest word between two") {
        Some(value) => println!("The longest word is {:?}", value),
        None => println!("The two words are equal!"),
    }

    println!("{:#?}", i);

    let s: &'static str = "I have a static lifetime.";
}

/////////////////////////////////////////////////////////////////////////////////

fn longest<'a, T>(x: &'a str, y: &'a str, ann: T) -> Option<&'a str>
where
    T: Display,
{
    println!("Annoncement: {}", ann);

    if x.len() > y.len() {
        Some(x)
    } else if x.len() < y.len() {
        Some(y)
    } else {
        None
    }
}
