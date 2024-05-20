pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> Self {
        Self(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

//////////////////////////////////////////////////

pub fn hello(name: &str) {
    println!("\nHello, {name} !");
}
