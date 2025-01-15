use std::fs::File;

mod ppm;
mod color;

fn main() {
    let mut file = File::create("image.ppm").unwrap();
    let ppm = ppm::PPMImage::default();

    ppm.print(&mut file);
}
