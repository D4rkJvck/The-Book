pub struct BreakFast {
    pub toast: String,
    _seasonal_fruit: String,
}
//---------------------------------------------------
impl BreakFast {
    pub fn summer(toast: String) -> BreakFast {
        BreakFast {
            toast: String::from(toast),
            _seasonal_fruit: String::from("peaches"),
        }
    }
}

///////////////////////////////////////////////////////

#[derive(Debug)]
pub enum Appetizer {
    Soup,
    Salad,
}

///////////////////////////////////////////////////////

fn _cook_order() {}

fn _fix_incorrect_order() {
    _cook_order();
    super::_deliver_order();
}

