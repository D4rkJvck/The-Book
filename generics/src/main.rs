use std::cmp::PartialOrd;

// enum Option<T> {
//     Some(T),
//     None,
// }

// impl<T> Option<T> {
//     fn unwrap_or_else<F>(self, f: F) -> T
//     where
//         F: FnOnce() -> T,
//     {
//         match self {
//             Some(x) => x,
//             None => f(),
//         }
//     }
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

////////////////////////////////////////////////////////////////////////////

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

////////////////////////////////////////////////////////////////////////////

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("\nThe largest number in {:?} is {}", number_list, result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("\nThe largest char in {:?} is {}", char_list, result);

    let p1 = Point { x: 5, y: 10.4 };

    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("\np3.x = {:?}\np3.y = {:?}", p3.x, p3.y);
}
