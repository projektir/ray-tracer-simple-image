use std::fmt;

#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
    z: i32
}

impl Point {
    pub fn new(x: i32, y: i32, z: i32) -> Point {
        Point { x, y, z }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_point_should_assign_xyz() {
        let point = Point::new(3, 4, 1);
        assert_eq!(3, point.x);
        assert_eq!(4, point.y);
        assert_eq!(1, point.z);
    }

    #[test]
    fn point_print_debug() {
        let point = Point::new(6, 8, -5);

        assert_eq!("Point { x: 6, y: 8, z: -5 }", format!("{:?}", point));
    }

    #[test]
    fn point_print_display() {
        let point = Point::new(6, 10, -5);

        assert_eq!("(6, 10, -5)", format!("{}", point));
    }
}
