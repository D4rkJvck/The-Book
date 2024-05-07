#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }

    fn height(&self) -> bool {
        self.height > 0
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

fn main() {
    let scale = 2;
    
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    
    let rect2 = Rectangle::square(20);
    
    println!(
        "rect1:
        Valid Width: {}
        Valid Height: {}
        Perimeter: {}
        Area: {}
        Can hold rect2: {}",
        rect1.width(),
        rect1.height(),
        rect1.perimeter(),
        rect1.area(),
        rect1.can_hold(&rect2)
    );
    
    println!(
        "rect2:
        Valid Width: {}
        Valid Height: {}
        Perimeter: {}
        Area: {}
        Can hold rect1: {}",
        rect2.width(),
        rect2.height(),
        rect2.perimeter(),
        rect2.area(),
        rect1.can_hold(&rect1)
    );
}

//////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}
