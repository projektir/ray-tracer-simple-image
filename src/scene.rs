use std::fmt;

use shape::Shape;

const DEFAULT_FOV: f32 = 100f32;

pub struct Scene {
    pub fov: f32,
    pub shapes: Vec<Box<Shape>>
}

impl Scene {
    pub fn new() -> Scene {
        Scene { fov: DEFAULT_FOV, shapes: Vec::new() }
    }
}

impl fmt::Display for Scene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FoV: {}\n", self.fov)?;

        for shape in self.shapes.iter() {
            write!(f, "{}\n", shape)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use shape::sphere::Sphere;
    use lin_alg::xyz::Xyz;

    #[test]
    fn scene_print_display() {
        let mut scene = Scene::new();
        
        let sphere1 = Sphere::new(Xyz::new(6.3, 10.0, -5.0), 22.1);
        let sphere2 = Sphere::new(Xyz::new(2.3, -3.0, -9.0), 10.0);

        scene.shapes.push(Box::new(sphere1));
        scene.shapes.push(Box::new(sphere2));

        assert_eq!("FoV: 100\n\
            Sphere: { center: (6.3, 10, -5), radius: 22.1 }\n\
            Sphere: { center: (2.3, -3, -9), radius: 10 }\n", format!("{}", scene));
    }
}
