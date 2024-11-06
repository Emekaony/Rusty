use core::fmt;
use std::ops::Add;

mod indepth_generics;
mod indepth_lifetimes;
mod learning_lifetimes;
mod ownership;
mod using_traits;
#[derive(Debug)] // print out nicely
struct Point {
    x: f64,
    y: f64,
}

impl Add for &Point {
    // need to give ur output a type
    type Output = Point;
    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// fn largest<T: PartialEq + std::cmp::PartialOrd>(items: &[T]) -> T {
//     let mut largest: &T = &items[0];
//     for item in items {
//         if *item > *largest {
//             largest = item
//         }
//     }
//     *largest
// }

fn main() {
    let v = vec![1, 2, 3, 4];
    let x = &v;
    for item in x {
        println!("item is {}", item)
    }
}
