#![allow(unused)]

use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize_author(&self) -> String {
        String::from("Unknown author")
    }

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x: {}", self.x);
        } else {
            println!("The larest number is y: {}", self.y);
        }
    }
}

// impl<T: Display> ToString for T {}

////////////////////////////////////////////////////////////////////////////////////////

// pub fn notify<T: Summary>(item1: &T, item2: &T) { //TODO: To Enforce same type
// pub fn notify(item: &(Summary + Display)) {
//pub fn notify<T: Summary + Display>(item: &T) {
pub fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summarize());
}

// fn some_function<T: Summary + Display, U: Clone + Debug>(t: &T, u: &U) -> i32 {
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Summary + Display,
    U: Clone + Debug,
{
    unimplemented!()
}

fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
