use std::path::Path;

use image::{ImageResult, RgbImage};
use num::NumCast;

use crate::math::color::Color;

pub struct Canvas {
    pub size: (u32, u32),
    image: RgbImage,
}

impl Canvas {
    pub fn new(size: (impl NumCast, impl NumCast)) -> Self {
        let size = (size.0.to_u32().unwrap(), size.1.to_u32().unwrap());
        Canvas {
            size,
            image: RgbImage::new(size.0, size.1),
        }
    }

    pub fn write_pixel(&mut self, x: i32, y: i32, color: Color) {
        let (width, height) = self.size;

        let x = width as i32 / 2 + x;
        let y = height as i32 / 2 - y - 1;

        self.image.put_pixel(x as u32, y as u32, color.into())
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> ImageResult<()> {
        self.image.save(path)
    }
}
