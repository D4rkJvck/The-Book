mod back_of_house;
mod front_of_house;
// mod customer;

pub use crate::front_of_house::hosting::{self, add_to_waitlist};

fn _deliver_order() {}

pub fn eat_at_restaurant() {
    add_to_waitlist();
    // seat_at_table();
    hosting::add_to_waitlist();
    hosting::seat_at_table();

    // Order a breakfast in summer with Rye toast
    let mut meal = back_of_house::BreakFast::summer(String::from("Rye"));

    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please!", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries")

    let mut order = back_of_house::Appetizer::Soup;
    dbg!(order);
    order = back_of_house::Appetizer::Salad;
    dbg!(order);
}
// }

////////////////////////////////////////////////

// #[cfg(test)]
// mod tests {

//     use super::*;
//     #[test]
//     fn it_works() {
//     }
// }
