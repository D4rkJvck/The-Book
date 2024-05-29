fn main() {
    let s = String::from("hello");
    take_ownership(s);

    let x = 5;
    make_copy(x);

    let s1 = give_ownership();
    println!("Given ownership: {}", s1);

    let s2 = transfer_ownership(s1);
    println!("Transfered ownership: {}", s2);

    let (s3, len) = calculate_length(s2);
    println!("Length of '{}': {} characters", s3, len);
}

//////////////////////////////////////////////////////

fn take_ownership(some_compound: String) {
    println!("Taken ownership: {}", some_compound);
}

fn make_copy(some_scalar: i32) {
    println!("Copied: {}", some_scalar);
}

fn give_ownership() -> String {
    let some_string = String::from("Take it!");
    some_string
}

fn transfer_ownership(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}
