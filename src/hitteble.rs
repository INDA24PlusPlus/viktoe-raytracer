use std::rc::Rc;

use nalgebra::Vector3;

use crate::{material::Material, Ray};

#[derive(Default)]
pub struct HittebleList {
    pub objects: Vec<Box<dyn Hitteble>>,
}

impl HittebleList {
    pub fn add(&mut self, obj: Box<dyn Hitteble>) {
        self.objects.push(obj);
    }
}

impl Hitteble for HittebleList {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for object in self.objects.iter() {
            if object.hit(ray, ray_tmin, closest_so_far, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.distance;
                *record = temp_record.clone();
            }
        }

        hit_anything
    }
}

pub trait Hitteble {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, record: &mut HitRecord) -> bool;
}

#[derive(Default, Clone)]
pub struct HitRecord {
    pub point: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub material: Material,
    pub distance: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3<f64>) {
        self.front_face = ray.direction.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
