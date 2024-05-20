use nodes::Node;
use std::rc::Rc;

fn main() {
    let leaf = Node::new(String::from("leaf"), 3);
    leaf.display_count();

    {
        let branch = Node::new(String::from("branch"), 5);
        branch.children.borrow_mut().push(Rc::clone(&leaf));
        println!("branch child: {:?}", branch.children);

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        branch.display_count();
        leaf.display_count();
    }

    println!("leaf parent: {:#?}", leaf.parent.borrow().upgrade());
    leaf.display_count();
}
