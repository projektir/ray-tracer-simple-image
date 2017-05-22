use std::fmt;

use shape::Shape;
use lin_alg::Square;
use lin_alg::xyz::Xyz;
use lin_alg::ray::Ray;

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

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        let origin = ray.origin.clone();
        let direction = ray.direction.clone();
        let center = self.center.clone();

        let to_center = origin - center;

        let a = direction.x.square() + direction.y.square() + direction.z.square();
        let b = (direction.x * to_center.x +
                 direction.y * to_center.y +
                 direction.z * to_center.z) * 2.0;
        let c = to_center.x.square() +
                to_center.y.square() +
                to_center.z.square() -
                self.radius.square();

        let discriminant = b.square() - 4.0 * a * c;

        if discriminant > 0.0 {
            let zero1 = (-b + discriminant.sqrt()) / (2.0 * a);
            let zero2 = (-b - discriminant.sqrt()) / (2.0 * a);

            if zero2 < zero1 {
                Some(zero2)
            } else {
                Some(zero1)
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lin_alg::EPSILON;

    #[test]
    fn new_should_assign_center_and_radius() {
        let xyz = Xyz::new(3.0, 4.4, 1.0);
        let sphere = Sphere::new(xyz.clone(), 22.1);

        assert_eq!(xyz, sphere.center);
        assert_eq!(22.1, sphere.radius);
        assert_eq!("Sphere { center: Xyz { x: 3, y: 4.4, z: 1 }, radius: 22.1 }",
            format!("{:?}", sphere));
    }

    #[test]
    fn print_display() {
        let sphere = Sphere::new(Xyz::new(6.3, 10.0, -5.0), 22.1);

        assert_eq!("Sphere: { center: (6.3, 10, -5), radius: 22.1 }", format!("{}", sphere));
    }

    #[test]
    fn intersect_yes() {
        let sphere = Sphere::new(Xyz::new(6.3, 10.0, -5.0), 22.1);
        let ray = Ray::new(Xyz::new(3.0, 4.4, 1.0), Xyz::new(5.0, -2.1, 2.0));

        let intersection = sphere.intersect(&ray);

        assert!(intersection.is_some());
        assert!((-3.727825 - intersection.unwrap()).abs() < EPSILON);
    }

    #[test]
    fn intersect_no() {
        let sphere = Sphere::new(Xyz::new(6.3, 10.0, -5.0), 1.1);
        let ray = Ray::new(Xyz::new(3.0, 4.4, 1.0), Xyz::new(5.0, -2.1, 2.0));

        let intersection = sphere.intersect(&ray);

        assert!(intersection.is_none());
    }
}
