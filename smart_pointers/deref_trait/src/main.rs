use deref_trait::*;

fn main() {
    {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(*y, 5);
        println!("\n{}", *y);
    }

    {
        let m = MyBox::new(String::from("Rust"));
        hello(&m);
    }
}
