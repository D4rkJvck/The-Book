fn print_coordonates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})\n");
}

fn main() {
    if let Some(x) = Some(5) {
        println!("\nResult after if let: {x}");
    }

    //---------------------------------------------------------------------

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "32".parse();

    if let Some(color) = favorite_color {
        println!("Favorite color with if let: {color}")
    } else if is_tuesday {
        println!("Today is Tuesday")
    } else if let Ok(age) = age {
        match age {
            age if age > 18 => println!("Adult with else if let"),
            _ => println!("Not Adult yet with else if let"),
        }
    } else {
        println!("Using blue as the background color")
    }

    println!();

    //---------------------------------------------------------------------

    let mut v = Vec::new();

    v.push("first");
    v.push("second");
    v.push("third");

    for (i, v) in v.iter().enumerate() {
        println!("{v} value at index {i} with for loop")
    }

    println!();

    while let Some(v) = v.pop() {
        println!("{v} value popped out with while let")
    }

    println!();

    //---------------------------------------------------------------------

    let point = (3, 5);
    print_coordonates(&point);
}
