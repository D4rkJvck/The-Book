use crate::List::{Cons, Nil};
use std::{borrow::Borrow, cell::RefCell, rc::Rc};

pub enum Node {
    Int(i32),
    Ref(Rc<RefCell<i32>>),
}

#[derive(Debug)]
pub enum List {
    Cons(Rc<RefCell<i32>>, RefCell<Rc<List>>),
    Nil,
}

impl List {
    pub fn new(nodes: Vec<Node>) -> RefCell<Rc<List>> {
        let mut list = RefCell::new(Rc::new(Nil));

        for node in nodes.into_iter().rev() {
            match node {
                Node::Int(i) => list = RefCell::new(Rc::new(Cons(Rc::new(RefCell::new(i)), list))),
                Node::Ref(p) => list = RefCell::new(Rc::new(Cons(p, list))),
            }
        }

        list
    }

    pub fn fmt(&self) -> Vec<i32> {
        let mut list = vec![];
        let mut current = self;

        while let List::Cons(i, next) = current {
            list.push(*i.borrow());
            current = next.borrow().borrow();
        }

        list
    }

    pub fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

pub fn ref_display(list: &List, list_ref: &Rc<List>) {
    println!(
        "List:\n{:?}\nOwnership count: {}\nNext item: {:?}\n",
        list,
        Rc::strong_count(list_ref),
        list.tail()
    );
}
