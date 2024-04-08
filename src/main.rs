use std::path::Path;
use crate::scene::builder::SceneBuilder;

mod renderer;
mod scene;
mod math;

fn main() {
    let scene_builder = scene::builder::BuiltIn::default();
    let scene = scene_builder.build();
    let file_path = Path::new("../result.ppm");
    match renderer::render_scene(&scene, file_path) {
        Ok(_) => println!("Image successfully created."),
        Err(why) => println!("Error: {}", why),
    }
}
