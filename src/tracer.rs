use image::{RgbImage, Rgb};

use scene::Scene;
use lin_alg::ray::Ray;
use lin_alg::xyz::Xyz;

pub fn trace_image(image: &mut RgbImage, scene: &Scene) {

    let base_ray = Ray::new(Xyz::new(0.0, 0.0, 0.0), Xyz::new(0.0, 0.0, 0.0));

    let pixel = trace_ray(&scene, base_ray.clone(), 20.0, 20.0);

    image.put_pixel(20, 20, pixel);
}

pub fn trace_ray(scene: &Scene, mut base_ray: Ray, x: f32, y: f32) -> Rgb<u8> {
    let direction = Xyz::screen_to_world(scene.width as f32, scene.height as f32, x, y, scene.near_clip, scene.fov);
    base_ray.direction = direction;

    Rgb { data: [0, 128, 255] }
}
