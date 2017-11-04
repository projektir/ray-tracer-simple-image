use std::fs::File;
use std::path::Path;

use serde_json;
use serde::Deserialize;

use image::{ImageBuffer, RgbImage, save_buffer, ColorType};

use tracer::trace_image;
use scene::Scene;
use shape::sphere::Sphere;

const FOV: &str = "FoV";
const SHAPES: &str = "shapes";

pub fn load_scene(path: &str) -> Scene {
    let path = Path::new(path);

    let file: File;
    
    file = match File::open(path) {
        Ok(file) => file,
        Err(_) => panic!("Unable to find file {:?}, make sure you are in the root directory of the project",
            path),
    };

    let scene_object: serde_json::Value = serde_json::from_reader(file).unwrap();

    let mut scene = Scene::new();

    if scene_object[FOV].is_number() {
        let fov: f32 = scene_object[FOV].as_f64().unwrap() as f32;
        scene.fov = fov;
    }

    if scene_object[SHAPES].is_array() {
        for shape in scene_object[SHAPES].as_array().unwrap().iter() {
            if let Ok(sphere) = Sphere::deserialize(shape) {
                scene.shapes.push(Box::new(sphere));
            }
        }
    }
    
    println!("\n{}", scene);

    scene
}

pub fn render_scene(scene: &Scene) {
    let image_vec = vec![0; 3 * (scene.width * scene.height) as usize];

    let mut img: RgbImage = ImageBuffer::from_raw(scene.width, scene.height, image_vec).unwrap();

    trace_image(&mut img, scene);

    save_buffer("test.png", &img, scene.width, scene.height, ColorType::RGB(8)).unwrap();
}
