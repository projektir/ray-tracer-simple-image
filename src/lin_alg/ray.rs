use std::fmt;

use lin_alg::point::Point;

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Point
}

impl Ray {
    pub fn new(origin: Point, direction: Point) -> Ray {
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
    fn new_ray_should_assign_origin_and_direction() {
        let origin = Point::new(3.0, 4.4, 1.0);
        let direction = Point::new(5.0, -2.1, 2.0);

        let ray = Ray::new(origin.clone(), direction.clone());

        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
        assert_eq!("Ray { origin: Point { x: 3, y: 4.4, z: 1 }, direction: Point { x: 5, y: -2.1, z: 2 } }",
            format!("{:?}", ray));
    }

    #[test]
    fn ray_print_display() {
        let ray = Ray::new(Point::new(3.0, 4.4, 1.0), Point::new(5.0, -2.1, 2.0));

        assert_eq!("Ray: { origin: (3, 4.4, 1), direction: (5, -2.1, 2) }",
            format!("{}", ray));
    }
}
