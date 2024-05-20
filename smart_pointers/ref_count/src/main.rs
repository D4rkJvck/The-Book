use ref_count::{
    ref_display,
    List::{self, Cons},
    Node::{Int, Ref},
};
use std::{cell::RefCell, rc::Rc};

fn main() {
    let value: Rc<RefCell<i32>> = Rc::new(RefCell::new(5));
    let cloned_value: Rc<RefCell<i32>> = Rc::clone(&value);

    let a = List::new(vec![Ref(cloned_value), Int(10)]);
    ref_display(&a, &a);

    let b = Rc::new(Cons(Rc::new(RefCell::new(3)), RefCell::new(Rc::clone(&a))));
    ref_display(&b, &a);

    {
        let c = Rc::new(Cons(Rc::new(RefCell::new(4)), RefCell::new(Rc::clone(&a))));
        ref_display(&c, &a);
        
        if let Some(link) = a.tail() {
            link.borrow_mut() = Rc::clone(&c);
        }

        println!("{:?} is now out of scope", c);
    }

    println!("Ownership count = {}", Rc::strong_count(&a));

    *value.borrow_mut() += 10;
    println!("\nLists after:\n{:?}\n{:?}", a.fmt(), b.fmt());

    // println!("Next item = {:?}", a.tail())
}
