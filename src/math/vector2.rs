use std::{fmt::Display, ops};

use num::NumCast;

#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: impl NumCast, y: impl NumCast) -> Self {
        Vector2 {
            x: x.to_f64().unwrap(),
            y: y.to_f64().unwrap(),
        }
    }

    pub fn angle_to(self, rhs: Self) -> f64 {
        (self.dot(rhs) / (self.magnitude() * rhs.magnitude())).acos()
    }

    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn magnitude(self) -> f64 {
        f64::sqrt(self.x.powf(2.0) + self.y.powf(2.0))
    }

    pub fn zero() -> Self {
        Vector2::new(0, 0)
    }
}

impl ops::Add<Self> for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Sub<Self> for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl ops::Mul<f64> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}

impl ops::Mul<Vector2> for f64 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Self::Output {
        Vector2::new(self * rhs.x, self * rhs.y)
    }
}

impl ops::Div<f64> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: f64) -> Self::Output {
        assert_ne!(rhs, num::zero());
        Vector2::new(self.x / rhs, self.y / rhs)
    }
}

impl Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector2 ⟨{}, {}⟩", self.x, self.y)
    }
}
