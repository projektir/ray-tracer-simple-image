use std::fs::File;
use std::io::prelude::*;
use std::path::{Path};

extern crate serde_json;

extern crate ray_tracer_simple_image;
use ray_tracer_simple_image::scene::Scene;
use ray_tracer_simple_image::shape::sphere::Sphere;

fn main() {
    let scenes_path = Path::new("scenes");
    let scene_01_path_buf = scenes_path.join("scene_01.json");

    let mut scene_json: String = String::new();
    let mut file: File;
    file = match File::open(scene_01_path_buf.as_path()) {
        Ok(file) => file,
        Err(_) => panic!("Unable to find file {:?}, make sure you are in the root directory of the project",
            scene_01_path_buf),
    };

    file.read_to_string(&mut scene_json).unwrap();

    let de_scene: serde_json::Value = serde_json::from_str(&scene_json).unwrap();

    let scene_array = de_scene.as_array().unwrap();

    let mut scene = Scene::new();

    for shape_object in scene_array.iter() {
        if shape_object.is_object() && shape_object["sphere"].is_object() {
            let sphere: Sphere = serde_json::from_str(&shape_object["sphere"].to_string()).unwrap();
            scene.shapes.push(Box::new(sphere));
        }
    }

    println!("Scene loaded:\n{}", scene);
}
