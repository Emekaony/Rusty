use core::fmt;
use std::ops::Add;

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

fn main() {
    ownership::run();
    learning_lifetimes::run();
}
