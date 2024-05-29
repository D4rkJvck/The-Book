pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn destructuring(&self) {
        println!("\nDestructuring Structs:");

        let Point { x, y, z } = self;
        println!("-> Value at X: {x}");
        println!("-> Value at Y: {y}");
        println!("-> Value at Z: {z}")
    }

    pub fn on_axis(&self) {
        match self {
            Point { x, y: 0, .. } => println!("-> On axis X at {x}"),
            Point { x: 0, y, .. } => println!("-> On axis Y at {y}"),
            Point { x, y, .. } => println!("-> On neither axis: ({x}, {y})"),
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[allow(unused)]
pub enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

#[allow(unused)]
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

impl Message {
    pub fn destructuring(&self) {
        println!("\nDestructuring Enums and Nested:");

        match self {
            Self::Quit => println!("-> No data to destructure, just quit!"),
            Self::Move { x, y } => println!("-> Move in new coords X: {x}, and Y: {y}"),
            Self::Write(txt) => println!("-> Text message: {txt}"),
            Self::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("-> Change color to red {r}, green {g} and blue {b}")
            }
            Self::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("-> Change color to hue {h}, saturation {s}, and value {v}")
            }
        };

        println!()
    }

    pub fn binding(&self) {
        println!("\n@ Bindings");

        match self {
            Self::Move {
                x: horizontal @ 3..=7,
                ..
            } => {
                println!("-> Found X in range: {horizontal}");
                // println!("{y}") // BUG: cannot find value `y` in this scope
            }
            Self::Move {
                y: vertical @ 3..=7,
                ..
            } => {
                // println!("{x}"); // BUG: cannot find value `y` in this scope
                println!("-> Found Y in range: {vertical}")
            }
            _ => (),
        }
    }
}
