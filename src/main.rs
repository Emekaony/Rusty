use std::ops::Add;
#[derive(Debug)] // print out nicely
struct Point {
    x: f64,
    y: f64,
}

impl Add for Point {
    // need to give ur output a type
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {}
