use std::fmt;

use shape::Shape;

const DEFAULT_WIDTH: u32 = 500;
const DEFAULT_HEIGHT: u32 = 500;
const DEFAULT_FOV: f32 = 100.0;
const DEFAULT_NEAR_CLIP: f32 = -1.0;

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f32,
    pub near_clip: f32,
    pub shapes: Vec<Box<Shape>>
}

impl Scene {
    pub fn new() -> Scene {
        Scene::default()
    }
}

impl fmt::Display for Scene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Width: {}\n", self.width)?;
        write!(f, "Height: {}\n", self.height)?;
        write!(f, "FoV: {}\n", self.fov)?;
        write!(f, "Near Clip: {}\n", self.near_clip)?;

        for shape in &self.shapes {
            write!(f, "{}\n", shape)?;
        }

        Ok(())
    }
}

impl Default for Scene {
    fn default() -> Scene {
        Scene {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            fov: DEFAULT_FOV,
            near_clip: DEFAULT_NEAR_CLIP,
            shapes: Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use shape::sphere::Sphere;
    use shape::material::Material;
    
    use lin_alg::xyz::Xyz;

    #[test]
    fn print_display() {
        let mut scene = Scene::new();
        
        let sphere1 = Sphere::new(Xyz::new(6.3, 10.0, -5.0), 22.1, Material::new());
        let sphere2 = Sphere::new(Xyz::new(2.3, -3.0, -9.0), 10.0, Material::new());

        scene.shapes.push(Box::new(sphere1));
        scene.shapes.push(Box::new(sphere2));

        assert_eq!("Width: 500\n\
            Height: 500\n\
            FoV: 100\n\
            Near Clip: -1\n\
            Sphere: { center: (6.3, 10, -5), radius: 22.1 }\n\
            Sphere: { center: (2.3, -3, -9), radius: 10 }\n", format!("{}", scene));
    }
}
