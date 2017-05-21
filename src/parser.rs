use std::fs::File;
use std::io::prelude::*;
use std::path::{Path};

extern crate serde_json;

use scene::Scene;
use shape::sphere::Sphere;

pub fn load_scene(path: &str) {
    let scenes_path = Path::new(path);

    let mut scene_json: String = String::new();
    let mut file: File;
    file = match File::open(scenes_path) {
        Ok(file) => file,
        Err(_) => panic!("Unable to find file {:?}, make sure you are in the root directory of the project",
            scenes_path),
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

    render_scene(scene);
}

pub fn render_scene(scene: Scene) {
    
}
