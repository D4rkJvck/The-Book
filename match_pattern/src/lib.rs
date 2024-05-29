use std::io;

pub fn prompt() -> i32 {
    println!("\nChoose a number:");

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("\nFail to read line");

    match num.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    }
}

////////////////////////////////////////////////////////////////////////////////

pub fn matching_literals(x: &i32) {
    println!("\nMatching Literals:");

    match x {
        3 => println!("-> All good things come in groups of three"),
        4 => println!("-> Death"),
        7 => println!("-> James Bond"),
        8 => println!("-> Religiously Auspicious"),
        13 => println!("-> Unlucky Friday!"),
        17 => println!("-> XVII -> VIXI"),
        26 => println!("-> Indians would try anything to avoid it!"),
        39 => println!("-> Morda Gow"),
        50 => println!("-> Curtis James Jackson III"),
        51 => println!("-> Extraterrestrial!!!"),
        444 => println!("-> Higher Purpose Spiritually"),
        666 => println!("-> Number of the Beast"),
        777 => println!("-> Holiest christian number"),
        1111 => println!("-> Take action and Get ready"),
        888888888 => println!("-> Bulgarian cursed phone number"),
        _ => println!("-> Nothing Special"),
    };
}

////////////////////////////////////////////////////////////////////////////////

pub fn matching_named_variables(x: Option<i32>) {
    println!("\nMatching Named Variables:");

    let y = 10;

    match x {
        Some(50) => println!("-> Got Fifty!"),
        Some(y) => println!("-> Matched, y = {y}"),
        _ => println!("-> Default case, x = {:?}", x),
    }

    println!("-> At the end x = {:?}, y = {y}", x);
}

////////////////////////////////////////////////////////////////////////////////

pub fn multiple_patterns(x: &i32) {
    println!("\nMultiple Patterns:");

    match x {
        0 => println!("-> Might be personal choice as well as a wrong input"),
        1 | 2 => println!("-> One or Two"),
        3 | 4 | 5 => println!("-> Three, Four or Five"),
        6 | 7 | 8 | 9 => println!("-> Six, Seven, Height or Nine"),
        _ => println!("-> At this point there is more than a digit"),
    }
}

////////////////////////////////////////////////////////////////////////////////

pub fn matching_ranges_of_values(x: &i32) {
    println!("\nMatching Ranges of Values with:");

    match x {
        0 => println!("-> Might be personal choice as well as a wrong input"),
        1..=9 => println!("-> Units"),
        10..=99 => println!("-> Tens"),
        100..=999 => println!("-> Hundreds"),
        _ => println!("-> Pretty High Range"),
    }
}

////////////////////////////////////////////////////////////////////////////////

pub fn only_uses_y(_: i32, y: i32) {
    println!("Only used {y}\n")
}

pub fn update_setting_value(old: &mut Option<i32>, new: Option<i32>) {
    match (&old, &new) {
        (Some(_), Some(_)) => println!("Can't overwrite an existing value!"),
        _ => *old = new,
    }
}

pub fn matching_named_variables_v2(x: Option<i32>) {
    println!("Matching Named Variables v2");

    let y = 10;

    match x {
        Some(50) => println!("-> Got Fifty!!!"),
        Some(n) if n == y => println!("-> Matched n, {n}"),
        _ => println!("-> Default case, x = {:?}", x),
    }
    println!("-> At the end: x = {:?}, y = {y}\n", x);
}
