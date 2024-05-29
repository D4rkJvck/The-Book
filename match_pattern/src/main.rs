mod models;

use match_pattern::{
    matching_literals, matching_named_variables, matching_named_variables_v2,
    matching_ranges_of_values, multiple_patterns, only_uses_y, prompt, update_setting_value,
};

use crate::models::{Color, Message, Point};

fn main() {
    let x = prompt();
    matching_literals(&x);
    matching_named_variables(Some(x));
    multiple_patterns(&x);
    matching_ranges_of_values(&x);

    // Destructuring to Break Apart Values
    {
        // Structs
        let p = Point::new(0, 7, 4);
        p.destructuring();
        p.on_axis();

        // Enums
        let m = Message::ChangeColor(Color::Rgb(0, 160, 255));
        m.destructuring();

        // Structs and Tuples
        let ((feet, inches), Point { x, y, z }) = (
            (3, 10),
            Point {
                x: 3,
                y: -10,
                z: 10,
            },
        );
        println!("Destructuring Structs and Tuples:");
        println!("-> {feet} feet, {inches} inches, at coords ({x}, {y}, {z})\n");
    }

    // Ignoring Values in a Pattern
    {
        //Ignoring an Entire Value with _
        only_uses_y(3, 4);

        // Ignoring Parts of a Value with a Nested _
        {
            let mut setting_value = None;
            let new_setting_value = Some(10);
            update_setting_value(&mut setting_value, new_setting_value);
            println!("Setting value is: {:?}", setting_value);
        }

        let numbers = (2, 4, 8, 16, 32);
        match numbers {
            (first, _, third, _, fifth) => {
                println!("Some numbers {first}, {third}, {fifth}\n")
            }
        }

        // Ignoring an Unused Variable by Starting Its Name with _
        {
            let s = Some(String::from("Hello!"));
            if let Some(_s) = s {
                println!("Found a string\n")
            }
            // println!("{:?}", s); //BUG: borrow of partially moved value: `s`
        }

        //Ignoring Remaining Parts of a Value with ..
        {
            let origin = Point { x: 0, y: 0, z: 0 };
            match origin {
                Point { x, .. } => println!("X is {x}"),
            }
        }

        match numbers {
            (first, .., last) => println!("Some numbers {first}, {last}\n"),
            // (.., second, ..) => println!("Some number {second}") // BUG: `..` can only be used once per tuple pattern
        }
    }

    // Extra Conditionals with Match Guards
    {
        let num = Some(4);
        match num {
            Some(x) if x % 2 == 0 => println!("The number {x} is even"),
            Some(x) => println!("The number {x} is odd"),
            None => (),
        }
        println!()
    }

    matching_named_variables_v2(Some(x));

    {
        let y = false;
        let x = 4;
        match x {
            4 | 5 | 6 if y => println!("-> Yes"),
            _ => println!("-> No"),
        }
    }

    // @ Binding
    let m = Message::Move { x: 5, y: 5 };
    m.binding();

    println!()
}
