use crate::material::Material;
use crate::HitRecord;
use crate::Hitteble;
use crate::Ray;
use nalgebra::Vector3;

pub struct Sphere {
    center: Vector3<f64>,
    radius: f64,
    material: Material
}

impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64, material: Material) -> Self {
        Sphere { center, radius, material}
    }
}

impl Hitteble for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, record: &mut HitRecord) -> bool {
        let oc = self.center - ray.origin;
        let a = ray.direction.magnitude_squared();
        let h = ray.direction.dot(&oc);
        let c = oc.magnitude_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }

        record.distance = root;
        record.point = ray.at(record.distance);
        let outward_normal = (record.point - self.center) / self.radius;
        record.set_face_normal(ray, outward_normal);
        record.material = self.material.clone();

        true
    }
}
