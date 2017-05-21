use std::fmt;

use shape::Shape;

pub struct Scene {
    pub shapes: Vec<Box<Shape>>
}

impl Scene {
    pub fn new() -> Scene {
        Scene { shapes: Vec::new() }
    }
}

impl fmt::Display for Scene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
    use lin_alg::point::Point;

    #[test]
    fn scene_print_display() {
        let mut scene = Scene::new();
        
        let sphere1 = Sphere::new(Point::new(6.3, 10.0, -5.0), 22.1);
        let sphere2 = Sphere::new(Point::new(2.3, -3.0, -9.0), 10.0);

        scene.shapes.push(Box::new(sphere1));
        scene.shapes.push(Box::new(sphere2));

        assert_eq!("Sphere: { center: (6.3, 10, -5), radius: 22.1 }\n\
            Sphere: { center: (2.3, -3, -9), radius: 10 }\n", format!("{}", scene));
    }
}
