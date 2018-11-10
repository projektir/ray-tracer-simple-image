pub mod material;
pub mod sphere;

use std::fmt::Display;

use self::material::Material;
use lin_alg::ray::Ray;

pub trait Shape: Display {
    fn get_material(&self) -> &Material;
    fn intersect(&self, ray: &Ray) -> Option<f32>;
}
