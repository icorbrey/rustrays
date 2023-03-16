use std::path::Path;

use image::{ImageResult, RgbImage};
use num::NumCast;

use crate::math::color::Color;

pub struct Canvas {
    pub width: u32,
    pub height: u32,
    image: RgbImage,
}

impl Canvas {
    /// Initializes a new canvas with the given width and height
    pub fn new(width: impl NumCast, height: impl NumCast) -> Self {
        let width = width.to_u32().unwrap();
        let height = height.to_u32().unwrap();
        Canvas {
            image: RgbImage::new(width, height),
            height,
            width,
        }
    }

    pub fn write_pixel(&mut self, x: i32, y: i32, color: Color) {
        let x = self.width as i32 / 2 + x;
        let y = self.height as i32 / 2 - y - 1;
        self.image.put_pixel(x as u32, y as u32, color.into())
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> ImageResult<()> {
        self.image.save(path)
    }
}
