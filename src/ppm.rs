use std::{fs::File, io::Write};

use crate::color::Color;

pub struct PPMImage {
    pub width: usize,
    pub height: usize,
}

impl Default for PPMImage {
    fn default() -> Self {
        Self::new(256, 256)
    }
}

impl PPMImage {
    pub fn new(width: usize, height: usize) -> Self {
        PPMImage { width, height }
    }

    pub fn print(&self, file: &mut File, image: Vec<Color>) {
        file.write_all(b"P3\n").unwrap();
        file.write_fmt(format_args!("{} {}\n", self.width, self.height))
            .unwrap();
        file.write_fmt(format_args!("{}\n", 255)).unwrap();

        for pixel in image {
            pixel.write(file);
        }
    }
}
