use std::fmt;

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

impl Xyz {
    pub fn new(x: f32, y: f32, z: f32) -> Xyz {
        Xyz { x, y, z }
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
    fn new_xyz_should_assign_xyz() {
        let xyz = Xyz::new(3.2, 4.0, -1.5);

        assert_eq!(3.2, xyz.x);
        assert_eq!(4.0, xyz.y);
        assert_eq!(-1.5, xyz.z);
        assert_eq!("Xyz { x: 3.2, y: 4, z: -1.5 }", format!("{:?}", xyz));
    }

    #[test]
    fn xyz_print_display() {
        let xyz = Xyz::new(6.3, 10.0, -5.0);

        assert_eq!("(6.3, 10, -5)", format!("{}", xyz));
    }
}
