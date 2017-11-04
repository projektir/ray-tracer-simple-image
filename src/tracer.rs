use image::{RgbImage, Rgb};

use scene::Scene;
use lin_alg::ray::Ray;
use lin_alg::xyz::Xyz;

pub fn trace_image(image: &mut RgbImage, scene: &Scene) {

    let mut base_ray = Ray::new(Xyz::new(0.0, 0.0, 0.0), Xyz::new(0.0, 0.0, 0.0));

    for x in 0..scene.width {
        for y in 0..scene.height {
            let pixel = trace_ray(scene, &mut base_ray, x as f32, y as f32);
            image.put_pixel(x, y, pixel);
        }
    }
}

pub fn trace_ray(scene: &Scene, ray: &mut Ray, x: f32, y: f32) -> Rgb<u8> {
    let direction = Xyz::screen_to_world(scene.width as f32, scene.height as f32, x, y,
        scene.near_clip, scene.fov);
    ray.direction = direction;

    let mut inter_dist: Option<f32> = None;
    let mut color = [0, 0, 0];

    for shape in &scene.shapes {
        let dist = shape.intersect(&*ray);

        if dist != None {
            if inter_dist == None || inter_dist < dist {
                inter_dist = dist;
                color = shape.get_material().color_diffuse;
            }
        }
    }

    Rgb { data: color }
}
