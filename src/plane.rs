use crate::material::Material;
use crate::HitRecord;
use crate::Hitteble;
use crate::Ray;
use nalgebra::Vector3;

pub struct Plane {
    point: Vector3<f64>,
    vec1: Vector3<f64>,
    vec2: Vector3<f64>,
    material: Material,
}

impl Plane {
    pub fn new(point: Vector3<f64>, vec1: Vector3<f64>, vec2: Vector3<f64>, material: Material) -> Self {
        Plane { point, vec1, vec2, material}
    }
}

impl Hitteble for Plane {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, record: &mut HitRecord) -> bool {
        let normal = self.vec1.cross(&self.vec2);
        if normal.dot(&ray.direction) == 0.0 {
            return false;
        }

        let distance =
            (self.point.dot(&normal) - ray.origin.dot(&normal)) / ray.direction.dot(&normal);

        if distance <= ray_tmin || ray_tmax <= distance {
            return false;
        }

        record.distance = distance;
        record.point = ray.at(record.distance);
        let outward_normal = normal;
        record.set_face_normal(ray, outward_normal);
        record.material = self.material.clone();

        true
    }
}
