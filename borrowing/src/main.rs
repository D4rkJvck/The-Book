fn main() {
    let mut s = String::from("hello");

    let len = calculate_length(&s);
    println!("Initial String: {}\nLength: {} characters", s, len);

    change(&mut s);
    println!("New String: {}\nLength: {} characters", s, len);

    let reference = no_dangle();
    println!("{}", reference);
}

////////////////////////////////////////////////////////////

fn calculate_length(some_string: &String) -> usize {
    some_string.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn no_dangle() -> String {
    let some_string = String::from("Rather than &String");
    some_string
}
