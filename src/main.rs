use std::fs::File;

use color::Color;
use nalgebra::{UnitVector3, Vector3};

mod color;
mod ppm;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    image_height = if image_height < 1 { 1 } else { image_height };

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Vector3::new(0.0, 0.0, 0.0);

    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u.scale(1.0 / image_width as f64);
    let pixel_delta_v = viewport_v.scale(1.0 / image_height as f64);

    let viewport_upper_left = camera_center
        - Vector3::new(0.0, 0.0, focal_length)
        - viewport_u.scale(1.0 / 2.0)
        - viewport_v.scale(1.0 / 2.0);

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut file = File::create("image.ppm").unwrap();
    let ppm = ppm::PPMImage::new(image_width as usize, image_height as usize);

    let mut image = Vec::new();

    for height in 0..image_height {
        println!("Scanlines remaining: {}", image_height - height);
        for width in 0..image_width {
            let pixel_center =
                pixel00_loc + (pixel_delta_u.scale(width as f64)) + (pixel_delta_v.scale(height as f64));
            let ray_direction = pixel_center - camera_center;

            let ray = Ray::new(camera_center, ray_direction);

            let color = ray_color(ray);

            image.push(color);
        }
    }

    println!("Done");

    ppm.print(&mut file, image);
}

fn ray_color(ray: Ray) -> Color {
    let unit_direction = UnitVector3::new_normalize(ray.direction);
    let a = 0.5 * (unit_direction.y + 1.0);
    let color = (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
    color
}

struct Ray {
    origin: Vector3<f64>,
    direction: Vector3<f64>
}

impl Ray {
    fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Self {
        Ray {
            origin,
            direction
        }
    }
}
