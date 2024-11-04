use core::fmt;
use std::ops::Add;
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

fn main() {
    let mut p1: Point = Point { x: 2.3, y: 1.9 };
    let p2: Point = Point { x: -2.2, y: 4.9 };

    println!("{} + {} is {}", p1, p2, &p1 + &p2);
    p1.x = 22.3;
}
