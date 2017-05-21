use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

extern crate serde_json;
use serde_json::value::Value;

extern crate ray_tracer_simple_image;
use ray_tracer_simple_image::scene::Scene;
use ray_tracer_simple_image::shape::Shape;
use ray_tracer_simple_image::shape::sphere::Sphere;
use ray_tracer_simple_image::point::Point;

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

    match de_scene {
        Value::Array(_) => {
            
        },
        _ => print!("Something else"),
    }
}
