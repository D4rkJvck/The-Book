#[derive(Debug)]
enum UsState {
    Alabama,
    // Alaska, Arizona, Arkansas,
    // California, Colorado, Connecticut,
    // Delaware,
    // Florida,
    // Georgia,
    // Hawaii,
    // Idaho, Illinois, Indiana, Iowa,
    // Kansas, Kentucky,
    // Louisiana,
    // Maine, Maryland, Massachusetts, Michigan, Minnesota, Mississippi, Missouri, Montana,
    // Nebraska, Nevada, NewHampshire, NewJersey, NewMexico, NewYork, NorthCarolina, NorthDakota,
    // Ohio, Oklahoma, Oregon,
    // Pennsylvania,
    // RhodeIsland,
    // SouthCarolina, SouthDakota,
    // Tennessee, Texas,
    // Utah,
    // Vermont, Virginia,
    // Washington, WestVirginia, Wisconsin, Wyoming,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let penny_to_cent = value_in_cents(Coin::Penny);
    let nickel_to_cent = value_in_cents(Coin::Nickel);
    let dime_to_cent = value_in_cents(Coin::Dime);
    let quarter_to_cent = value_in_cents(Coin::Quarter(UsState::Alabama));

    println!(
        "Conversion:
        Penny to cent: {penny_to_cent}
        Nickel to cent: {nickel_to_cent}
        Dime to cent: {dime_to_cent}
        Quarter to cent: {quarter_to_cent}"
    );

    let five = Some(5);
    let six = plus_one(five);
    let _none = plus_one(None);

    dbg!(six);

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

//////////////////////////////////////////////////////////

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
