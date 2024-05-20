use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct Node {
    pub id: String,
    pub parent: RefCell<Weak<Node>>,
    pub value: i32,
    pub children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    pub fn new(id: String, value: i32) -> Rc<Self> {
        Rc::new(Self {
            id,
            parent: RefCell::new(Weak::new()),
            value,
            children: RefCell::new(vec![]),
        })
    }

    pub fn display_count(self: &Rc<Node>) {
        println!(
            "{} strong = {}, weak = {}",
            self.id,
            Rc::strong_count(self),
            Rc::weak_count(self)
        )
    }
}
