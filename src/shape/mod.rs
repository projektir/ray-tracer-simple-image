pub mod sphere;

use std::fmt;

use lin_alg::ray::Ray;

pub trait Shape: fmt::Display {
    fn intersect(&self, ray: &Ray) -> f32;
}
