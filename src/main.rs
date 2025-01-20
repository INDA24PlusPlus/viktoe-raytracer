
use camera::Camera;
use hitteble::{HitRecord, Hitteble, HittebleList};
use nalgebra::Vector3;
use plane::Plane;
use sphere::Sphere;

mod color;
mod hitteble;
mod plane;
mod ppm;
mod sphere;
mod camera;

fn main() {
    let mut world = HittebleList::default();

    world.add(Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Plane::new(
        Vector3::new(0.0, -0.5, 0.0),
        Vector3::new(1.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 1.0),
    )));

    let mut camera = Camera::default();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;

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
