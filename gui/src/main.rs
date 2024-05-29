mod components;

use crate::components::{button::Button, select_box::SelectBox, text_field::TextField};
use gui::Screen;

fn main() {
    let screen = Screen::new(vec![
        Box::new(Button::new(50, 10, String::from("OK"))),
        Box::new(TextField::new(
            100,
            20,
            String::from("Search"),
            String::from("Enter text here"),
        )),
        Box::new(SelectBox::new(
            75,
            10,
            vec![
                String::from("Yes"),
                String::from("Maybe"),
                String::from("No"),
            ],
        )),
    ]);

    screen.run();
}
