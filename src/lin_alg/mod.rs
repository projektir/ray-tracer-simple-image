pub mod ray;
pub mod xyz;

pub const EPSILON: f32 = 0.00001;

pub trait Square {
    fn square(&self) -> f32;
}

impl Square for f32 {
    fn square(&self) -> f32 {
        self * self
    }
}
