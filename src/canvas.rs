use std::path::Path;

use image::{ImageResult, RgbImage};
use num::{NumCast, ToPrimitive};

use crate::math::{Color, Point};

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

    pub fn write_pixel(&mut self, position: Point, color: Color) {
        let position = Point::new(
            self.size.0.to_i32().unwrap() / 2 + position.x,
            self.size.1.to_i32().unwrap() / 2 - position.y - 1,
        );
        self.image
            .put_pixel(position.x as u32, position.y as u32, color.into())
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> ImageResult<()> {
        self.image.save(path)
    }
}
