use std::fmt;

use lin_alg::xyz::Xyz;

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Xyz,
    pub direction: Xyz
}

impl Ray {
    pub fn new(origin: Xyz, direction: Xyz) -> Ray {
        Ray { origin, direction }
    }
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ray: {{ origin: {}, direction: {} }}", self.origin, self.direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_assign_origin_and_direction() {
        let origin = Xyz::new(3.0, 4.4, 1.0);
        let direction = Xyz::new(5.0, -2.1, 2.0);

        let ray = Ray::new(origin.clone(), direction.clone());

        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
        assert_eq!("Ray { origin: Xyz { x: 3, y: 4.4, z: 1 }, direction: Xyz { x: 5, y: -2.1, z: 2 } }",
            format!("{:?}", ray));
    }

    #[test]
    fn print_display() {
        let ray = Ray::new(Xyz::new(3.0, 4.4, 1.0), Xyz::new(5.0, -2.1, 2.0));

        assert_eq!("Ray: { origin: (3, 4.4, 1), direction: (5, -2.1, 2) }",
            format!("{}", ray));
    }
}
