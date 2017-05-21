use std::fmt;

use shape::Shape;
use lin_alg::xyz::Xyz;

#[derive(Debug, Serialize, Deserialize)]
pub struct Sphere {
    center: Xyz,
    radius: f32
}

impl Sphere {
    pub fn new(center: Xyz, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl fmt::Display for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sphere: {{ center: {}, radius: {} }}", self.center, self.radius)
    }
}

impl Shape for Sphere { }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sphere_should_assign_center_and_radius() {
        let xyz = Xyz::new(3.0, 4.4, 1.0);
        let sphere = Sphere::new(xyz.clone(), 22.1);

        assert_eq!(xyz, sphere.center);
        assert_eq!(22.1, sphere.radius);
        assert_eq!("Sphere { center: Xyz { x: 3, y: 4.4, z: 1 }, radius: 22.1 }",
            format!("{:?}", sphere));
    }

    #[test]
    fn sphere_print_display() {
        let sphere = Sphere::new(Xyz::new(6.3, 10.0, -5.0), 22.1);

        assert_eq!("Sphere: { center: (6.3, 10, -5), radius: 22.1 }", format!("{}", sphere));
    }
}
