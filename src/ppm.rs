use std::{fs::File, io::Write};

use crate::color::Color;

pub struct PPMImage {
    width: usize,
    height: usize,
    color_range: usize,
}

impl Default for PPMImage {
    fn default() -> Self {
        Self::new(256, 256, 255)
    }
}

impl PPMImage {
    fn new(width: usize, height: usize, color_range: usize) -> Self {
        PPMImage {
            width,
            height,
            color_range,
        }
    }

    pub fn print(&self, file: &mut File) {
        file.write_all(b"P3\n").unwrap();
        file.write_fmt(format_args!("{} {}\n", self.width, self.height))
            .unwrap();
        file.write_fmt(format_args!("{}\n", self.color_range))
            .unwrap();

        for height in 0..self.height {
            println!("Scanlines remaining: {}", self.height - height);
            for width in 0..self.width {
                let color = Color::new((width as f64 / (self.width - 1) as f64 * 256.0) as u8, (height as f64 / (self.height) as f64 * 256.0) as u8, 0);
                color.write(file);
            }
        }
        println!("Done");
    }
}
