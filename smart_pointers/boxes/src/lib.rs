#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    pub fn fmt(&self) -> Vec<i32> {
        let mut list = vec![];
        let mut current = self;

        while let List::Cons(i, next) = current {
            list.push(*i);
            current = next;
        }

        list
    }
}
