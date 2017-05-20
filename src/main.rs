extern crate ray_tracer_simple_image;
use ray_tracer_simple_image::point::Point;
use ray_tracer_simple_image::shape::sphere::Sphere;

fn main() {
    let sphere = Sphere::new(Point::new(4.0, 2.0, -7.0), 22.1);

    println!("Hello, world, here's a sphere: {}", sphere);
}
