use std::fs::File;

use color::Color;
use hitteble::{HitRecord, Hitteble, HittebleList};
use nalgebra::{UnitVector3, Vector3};
use plane::Plane;
use rand::{random, thread_rng, Rng};
use sphere::Sphere;

mod color;
mod hitteble;
mod plane;
mod ppm;
mod sphere;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;

    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    image_height = if image_height < 1 { 1 } else { image_height };

    let mut world = HittebleList::default();

    world.add(Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Plane::new(
        Vector3::new(0.0, -0.5, 0.0),
        Vector3::new(1.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 1.0),
    )));

    let light = vec![Vector3::new(10.0, 10.0, 5.0)];

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Vector3::new(0.0, 0.0, 0.0);

    let depth = 10;
    let samples_per_pixel = 10;
    let pixel_sample_scale = 1.0 / samples_per_pixel as f64;

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
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let offset = Vector3::new(random::<f64>() - 0.5, random::<f64>() - 0.5, 0.0);
                let pixel_sample = pixel00_loc
                    + ((pixel_delta_u).scale(width as f64 + offset.x))
                    + (pixel_delta_v.scale(height as f64 + offset.y));

                let ray_direction = pixel_sample - camera_center;

                let ray = Ray::new(camera_center, ray_direction);

                pixel_color = pixel_color + ray_color(&ray, depth, &world, &light);
            }

            image.push(pixel_color * pixel_sample_scale);
        }
    }

    println!("Done");

    ppm.print(&mut file, image);
}

fn ray_color(ray: &Ray, depth: i32, world: &impl Hitteble, light: &Vec<Vector3<f64>>) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut record = HitRecord::default();

    let color_from_scatter = if world.hit(ray, 0.001, f64::MAX, &mut record) {
        let direction = random_on_hemisphere(record.normal);
        0.5 * ray_color(&Ray::new(record.point, direction), depth - 1, world, light)
    } else {
        Color::new(0.0, 0.0, 0.0)
    };

    let ray_to_light = Ray::new(record.point, light[0] - record.point );
    let color_from_light = if record.distance != 0.0 && !world.hit(&ray_to_light, 0.001, f64::MAX, &mut HitRecord::default()) {
        // let unit_direction = UnitVector3::new_normalize(ray.direction);
        // let a = 0.5 * (unit_direction.y + 1.0);
        // (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
        Color::new(0.9, 0.9, 0.7)
    } else {
        Color::new(0.0, 0.0, 0.0)
    };

    color_from_scatter + color_from_light

    // let unit_direction = UnitVector3::new_normalize(ray.direction);
    // let a = 0.5 * (unit_direction.y + 1.0);
    // (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

struct Ray {
    origin: Vector3<f64>,
    direction: Vector3<f64>,
}

impl Ray {
    fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Self {
        Ray { origin, direction }
    }

    fn at(&self, t: f64) -> Vector3<f64> {
        self.origin + t * self.direction
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

fn random_unit_vector() -> Vector3<f64> {
    loop {
        let p = random_vector_range(-1.0, 1.0);
        let lensq = p.magnitude_squared();
        if lensq <= 1.0 {
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
