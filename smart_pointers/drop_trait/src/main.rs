use drop_trait::CustomSmartPointer;

fn main() {
    let c = CustomSmartPointer::new(String::from("his stuff"));
    println!(
        "\nCustom Smart Pointer with data {:?} created !",
        c.get_data()
    );

    let d = CustomSmartPointer::new(String::from("my stuff"));
    println!(
        "Custom Smart Pointer with data {:?} created !",
        d.get_data()
    );

    // d.drop();    // BUG: explicit use of destructor method...

    drop(c);
    println!("\nCustom Smart Pointer dropped before end of main function !");
}
