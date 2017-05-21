extern crate ray_tracer_simple_image;
use ray_tracer_simple_image::parser::{load_scene, render_scene};

fn main() {
    let scene = load_scene("scenes/scene_01.json");

    render_scene(scene, 50, 50);   
}
