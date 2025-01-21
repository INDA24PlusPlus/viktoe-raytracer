use camera::Camera;
use color::Color;
use hitteble::{HitRecord, Hitteble, HittebleList};
use material::Material;
use nalgebra::Vector3;
use plane::Plane;
use sphere::Sphere;
use material::MaterialType::{Labertian, Metal};

mod camera;
mod color;
mod hitteble;
mod material;
mod plane;
mod ppm;
mod sphere;

fn main() {
    let mut world = HittebleList::default();

    let material_ground = Material::new(Color::new(0.8, 0.8, 0.0), Labertian);
    let material_center = Material::new(Color::new(0.1, 0.2, 0.5), Labertian);
    let material_left = Material::new(Color::new(0.8, 0.8, 0.8), Metal);
    let material_right = Material::new(Color::new(0.8, 0.6, 0.2), Metal);

    world.add(Box::new(Sphere::new(
        Vector3::new(0.0, 0.0, -1.0),
        0.5,
        material_center
    )));
    world.add(Box::new(Sphere::new(
        Vector3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left
    )));
    world.add(Box::new(Sphere::new(
        Vector3::new(1.0, 0.0, -1.0),
        0.5,
        material_right
    )));
    world.add(Box::new(Plane::new(
        Vector3::new(0.0, -0.5, 0.0),
        Vector3::new(1.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 1.0),
        material_ground
    )));

    let mut camera = Camera::default();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.render(world);
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
