use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use serde_json;

use image::{ImageBuffer, RgbaImage, GenericImage, save_buffer, ColorType};

use scene::Scene;
use shape::sphere::Sphere;

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
        if shape_object.is_object() && shape_object["sphere"].is_object() {
            let sphere: Sphere = serde_json::from_str(&shape_object["sphere"].to_string()).unwrap();
            scene.shapes.push(Box::new(sphere));
        }
    }
    
    scene
}

pub fn render_scene(scene: Scene, width: u32, height: u32) {
    let mut image_vec = vec![0; 4 * (width * height) as usize];

    let img: RgbaImage = ImageBuffer::from_raw(width, height, image_vec).unwrap();



    let ref mut fout = File::create(&Path::new("test.png")).unwrap();
    let _ = save_buffer("test.png", &img, width, height, ColorType::RGBA(8)).unwrap();
}
