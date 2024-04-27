fn main() {
    let s = String::from("Hello there !");

    let first = first_word(&s);
    println!("First word: {}", first);

    let second = second_word(&s);
    println!("Second word: {}", second);

    let tab = [1, 2, 3, 4, 5];
    let slice = &tab[1..3];
    assert_eq!(slice, [2, 3]);
}

/////////////////////////////////////////////////////

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut index: usize = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if index != 0 {
                return &s[index..i];
            } else {
                index = i + 1
            }
        }
    }

    &s[..]
}
