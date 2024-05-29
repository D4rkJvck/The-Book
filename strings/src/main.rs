fn main() {
    let mut s = String::new();
    println!("1 -> {:?}", s);

    let s1 = "tic".to_string();

    s.push_str(&s1);
    s.push(' ');
    println!("2 -> {s}");

    let s2 = String::from("tac");

    s += &s2;
    s.push(' ');
    println!("3 -> {s}");

    let s3 = "toe";

    s = format!("{s1}-{s2}-{s3}"); // No ownership taken here...

    println!("4 -> {s}");

    s = String::from("Здравствуйте");

    println!("5 ->");
    for c in s.chars() {
        if c == s.chars().last().unwrap() {
            println!("{c}");
            continue;
        }
        print!("{c} ");
    }

    println!("6 ->");
    for b in s.bytes() {
        if b == s.bytes().last().unwrap() {
            print!(" ");
            continue;
        }
        print!("{b} ");
    }
}
