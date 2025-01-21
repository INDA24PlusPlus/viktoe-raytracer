use nalgebra::Vector3;

use crate::{camera::random_unit_vector, color::Color, hitteble::HitRecord, Ray};

#[derive(Default, Clone)]
pub struct Material {
    albedo: Color,
    material_type: MaterialType
}

#[derive(Default, Clone)]
pub enum MaterialType {
    #[default] Labertian,
    Metal
}

impl Material {
    pub fn new(albedo: Color, material_type: MaterialType) -> Self {
        Material {
            albedo,
            material_type
        }
    }
    pub fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        match self.material_type {
            MaterialType::Labertian => {
                let mut scatter_direction = record.normal + random_unit_vector();
                if near_zero(&scatter_direction) {
                    scatter_direction = record.normal;
                }
                let scattered = Ray::new(record.point, scatter_direction);
                let attenuation = self.albedo.clone();

                Some((scattered, attenuation))
            },
            MaterialType::Metal => {
                let reflected = reflect(&ray_in.direction, &record.normal);
                let scattered = Ray::new(record.point, reflected);
                let attenuation = self.albedo.clone();
                Some((scattered, attenuation))
            }
        }
    }
}

fn near_zero(vector: &Vector3<f64>) -> bool {
    vector.iter().all(|x| *x < 1e-8)
}

fn reflect(vector: &Vector3<f64>, normal: &Vector3<f64>) -> Vector3<f64> {
    vector - 2.0 * vector.dot(normal) * normal
}
