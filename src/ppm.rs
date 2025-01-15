use std::{fs::File, io::Write};

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
                let red = width as f64 / (self.width - 1) as f64;
                let green = height as f64 / (self.height - 1) as f64;
                let blue = 0;

                file.write_fmt(format_args!(
                    "{} {} {}\n",
                    red * self.color_range as f64,
                    green * self.color_range as f64,
                    blue * self.color_range
                ))
                .unwrap()
            }
        }
        println!("Done");
    }
}
