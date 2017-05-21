use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use serde_json;

use image::{ImageBuffer, RgbImage, Rgb, save_buffer, ColorType};

use scene::Scene;
use shape::sphere::Sphere;
use lin_alg::ray::Ray;
use lin_alg::xyz::Xyz;

pub fn load_scene(path: &str) -> Scene {
    let path = Path::new(path);

    let mut file: File;
    
    file = match File::open(path) {
        Ok(file) => file,
        Err(_) => panic!("Unable to find file {:?}, make sure you are in the root directory of the project",
            path),
    };

    let mut scene_string: String = String::new();
    file.read_to_string(&mut scene_string).unwrap();

    let scene_json: serde_json::Value = serde_json::from_str(&scene_string).unwrap();
    let scene_array = scene_json.as_array().unwrap();

    let mut scene = Scene::new();

    for shape_object in scene_array.iter() {
        println!("{}", shape_object);

        const SPHERE: &str = "sphere";
        const FOV: &str = "FoV";

        if shape_object.is_object() {
            if shape_object[SPHERE].is_object() {
                let sphere: Sphere = serde_json::from_str(&shape_object[SPHERE].to_string()).unwrap();
                scene.shapes.push(Box::new(sphere));
            }

            if shape_object[FOV].is_number() {
                let fov: f32 = serde_json::from_str(&shape_object[FOV].to_string()).unwrap();
                scene.fov = fov;
            }
        }
    }
    
    println!("\n{}", scene);

    scene
}

pub fn render_scene(scene: Scene, width: u32, height: u32) {
    let mut image_vec = vec![0; 3 * (width * height) as usize];

    let mut img: RgbImage = ImageBuffer::from_raw(width, height, image_vec).unwrap();

    trace_image(&mut img);

    let _ = save_buffer("test.png", &img, width, height, ColorType::RGB(8)).unwrap();
}

pub fn trace_image(image: &mut RgbImage) {
    let (width, height) = image.dimensions();

    let base_ray = Ray::new(Xyz::new(0.0, 0.0, 0.0), Xyz::new(0.0, 0.0, 0.0));

    let pixel = trace_ray(base_ray.clone(), 20.0, 20.0);

    image.put_pixel(20, 20, pixel);
}

pub fn trace_ray(mut base_ray: Ray, x: f32, y: f32) -> Rgb<u8> {
    base_ray.direction = Xyz::new(2.0, 4.0, 2.0);

    Rgb { data: [0, 128, 255] }
}
