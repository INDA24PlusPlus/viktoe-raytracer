use std::fs::File;

use color::Color;

mod color;
mod ppm;

fn main() {
    let mut file = File::create("image.ppm").unwrap();
    let ppm = ppm::PPMImage::default();

    let mut image = Vec::new();

    for height in 0..ppm.height {
        println!("Scanlines remaining: {}", ppm.height - height);
        for width in 0..ppm.width {
            let color = Color::new(
                (width as f64 / (ppm.width - 1) as f64 * 256.0) as u8,
                (height as f64 / (ppm.height) as f64 * 256.0) as u8,
                0,
            );
            image.push(color);
        }
    }
    println!("Done");
    ppm.print(&mut file, image);
}
