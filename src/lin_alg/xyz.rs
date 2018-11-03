use std::fmt;
use std::ops::Sub;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Xyz {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl PartialEq for Xyz {
    fn eq(&self, other: &Xyz) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z
    }
}

impl Sub for Xyz {
    type Output = Xyz;

    fn sub(self, other: Xyz) -> Xyz {
        Xyz::new(self.x - other.x,
                 self.y - other.y,
                 self.z - other.z)
    }
}

impl Xyz {
    pub fn new(x: f32, y: f32, z: f32) -> Xyz {
        Xyz { x, y, z }
    }

    pub fn length(&self) -> f32 {
        (self.x.square() + self.y.square() + self.z.square()).sqrt()
    }

    pub fn normalize(&self) -> Xyz {
        let length = self.length();
        Xyz::new(self.x / length, self.y / length, self.z / length)
    }

    pub fn screen_to_world(screen_width: f32, screen_height: f32, screen_x: f32, screen_y: f32, world_z: f32, fov: f32) -> Xyz
    {
        let fov = fov.to_radians();
        let tan = (fov / 2.0).tan();

        let x_ratio = screen_width / (2.0 * tan);
        let y_ratio = screen_height / (2.0 * tan);

        let world_x = ((screen_x / x_ratio) - tan ) * world_z.abs();
        let world_y = ((screen_y / y_ratio) - tan ) * world_z.abs() * -1.0;

        Xyz::new(world_x, world_y, world_z)
    }
}

impl fmt::Display for Xyz {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_assign_xyz() {
        let xyz = Xyz::new(3.2, 4.0, -1.5);

        assert_eq!(3.2, xyz.x);
        assert_eq!(4.0, xyz.y);
        assert_eq!(-1.5, xyz.z);
        assert_eq!("Xyz { x: 3.2, y: 4.0, z: -1.5 }", format!("{:?}", xyz));
    }

    #[test]
    fn print_display() {
        let xyz = Xyz::new(6.3, 10.0, -5.0);

        assert_eq!("(6.3, 10, -5)", format!("{}", xyz));
    }

    #[test]
    fn length() {
        let xyz = Xyz::new(1.1, 5.0, -3.2);

        assert!((6.03738 - xyz.length()).abs() < EPSILON);
    }

    #[test]
    fn normalize() {
        let xyz = Xyz::new(1.1, 5.0, -3.2).normalize();

        assert!((0.182198 - xyz.x).abs() < EPSILON);
        assert!((0.828173 - xyz.y).abs() < EPSILON);
        assert!((-0.530031 - xyz.z).abs() < EPSILON);
    }

    #[test]
    fn screen_to_world() {
        let xyz = Xyz::screen_to_world(300.0, 300.0, 50.0, 30.0, -1.0, 100.0);

        assert!((-0.79450244 - xyz.x).abs() < EPSILON);
        assert!((0.9534029 - xyz.y).abs() < EPSILON);
        assert!((-1.0 - xyz.z).abs() < EPSILON);
    }
}
