use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z
    }
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
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
        let point = Point::new(3.2, 4.0, -1.5);

        assert_eq!(3.2, point.x);
        assert_eq!(4.0, point.y);
        assert_eq!(-1.5, point.z);
        assert_eq!("Point { x: 3.2, y: 4, z: -1.5 }", format!("{:?}", point));
    }

    #[test]
    fn point_print_display() {
        let point = Point::new(6.3, 10.0, -5.0);

        assert_eq!("(6.3, 10, -5)", format!("{}", point));
    }
}
