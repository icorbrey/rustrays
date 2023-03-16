use std::{fmt::Display, ops};

use num::NumCast;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: impl NumCast, y: impl NumCast) -> Self {
        Point {
            x: x.to_i32().unwrap(),
            y: y.to_i32().unwrap(),
        }
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point ⟨{}, {}⟩", self.x, self.y)
    }
}
