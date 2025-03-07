use std::fs::File;
use nalgebra::UnitVector3;
use rand::thread_rng;
use rand::Rng;

use nalgebra::Vector3;
use rand::random;

use crate::{color::Color, hitteble::{HitRecord, Hitteble, HittebleList}, Ray};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    image_height: i32,
    center: Vector3<f64>,
    pixel00_loc: Vector3<f64>,
    pixel_delta_u: Vector3<f64>,
    pixel_delta_v: Vector3<f64>,
    pixel_sample_scale: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            samples_per_pixel: 10,
            max_depth: 10,
            image_height: 0,
            center: Vector3::default(),
            pixel00_loc: Vector3::default(),
            pixel_delta_u: Vector3::default(),
            pixel_delta_v: Vector3::default(),
            pixel_sample_scale: 0.0,
        }
    }
}

impl Camera {
    pub fn render(&mut self, world: HittebleList) {
        self.initialize();

        let mut file = File::create("image.ppm").unwrap();
        let ppm = crate::ppm::PPMImage::new(self.image_width as usize, self.image_height as usize);

        let mut image = Vec::new();

        for height in 0..self.image_height {
            println!("Scanlines remaining: {}", self.image_height - height);
            for width in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(width, height);

                    pixel_color = pixel_color + self.ray_color(&ray, self.max_depth, &world);
                }

                image.push(pixel_color * self.pixel_sample_scale);
            }
        }

        println!("Done");

        ppm.print(&mut file, image);
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 { 1 } else { self.image_height };

        self.pixel_sample_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = Vector3::new(0.0, 0.0, 0.0);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u.scale(1.0 / self.image_width as f64);
        self.pixel_delta_v = viewport_v.scale(1.0 / self.image_height as f64);

        let viewport_upper_left = self.center
            - Vector3::new(0.0, 0.0, focal_length)
            - viewport_u.scale(1.0 / 2.0)
            - viewport_v.scale(1.0 / 2.0);

        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(&mut self, ray: &Ray, depth: i32, world: &impl Hitteble) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut record = HitRecord::default();

        if !world.hit(ray, 0.001, f64::MAX, &mut record) {
            // let unit_direction = UnitVector3::new_normalize(ray.direction);
            // let a = 0.5 * (unit_direction.y + 1.0);
            // return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
            return Color::new(0.0, 0.0, 0.0);
        }

        let color_from_emission = record.material.emitted();

        if let Some((scattered, attenuation)) = record.material.scatter(ray, &record) {
            let color_from_scatter = attenuation * self.ray_color(&scattered, depth -1, world);

            color_from_emission + color_from_scatter
        } else {
            color_from_emission
        }

    }

    fn get_ray(&self, width: i32, height: i32) -> Ray {
        let offset = Vector3::new(random::<f64>() - 0.5, random::<f64>() - 0.5, 0.0);
        let pixel_sample = self.pixel00_loc
            + ((self.pixel_delta_u).scale(width as f64 + offset.x))
            + (self.pixel_delta_v.scale(height as f64 + offset.y));

        let ray_direction = pixel_sample - self.center;

        Ray::new(self.center, ray_direction)
    }
}

fn random_vector() -> Vector3<f64> {
    Vector3::new(random(), random(), random())
}

fn random_vector_range(min: f64, max: f64) -> Vector3<f64> {
    Vector3::new(
        thread_rng().gen_range(min..max),
        thread_rng().gen_range(min..max),
        thread_rng().gen_range(min..max),
    )
}

pub fn random_unit_vector() -> Vector3<f64> {
    loop {
        let p = random_vector_range(-1.0, 1.0);
        let lensq = p.magnitude_squared();
        if 1e-160 < lensq && lensq <= 1.0 {
            return p / lensq.sqrt();
        }
    }
}

fn random_on_hemisphere(normal: Vector3<f64>) -> Vector3<f64> {
    let on_unit_sphere = random_unit_vector();
    if on_unit_sphere.dot(&normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}
