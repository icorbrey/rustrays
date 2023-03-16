use std::{fmt::Display, ops};

use image::Rgb;

#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }
}

impl ops::Add for Color {
    type Output = Color;

    fn add(self, right: Self) -> Self::Output {
        Color::new(self.r + right.r, self.g + right.g, self.b + right.b)
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, right: f64) -> Self::Output {
        Color::new(
            (self.r as f64 * right).clamp(u8::MIN as f64, u8::MAX as f64) as u8,
            (self.g as f64 * right).clamp(u8::MIN as f64, u8::MAX as f64) as u8,
            (self.b as f64 * right).clamp(u8::MIN as f64, u8::MAX as f64) as u8,
        )
    }
}

impl Into<Rgb<u8>> for Color {
    fn into(self) -> Rgb<u8> {
        Rgb([self.r, self.g, self.b])
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Color {{ r: {}, g: {}, b: {} }}", self.r, self.g, self.b)
    }
}
