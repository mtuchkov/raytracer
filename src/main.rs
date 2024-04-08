use crate::scene::builder::SceneBuilder;

mod renderer;
mod scene;
mod math;

fn main() {
    let scene_builder = scene::builder::BuiltIn::random();
    let scene = scene_builder.build();
    match renderer::render_scene(&scene, "../result.ppm".to_string()) {
        Ok(_) => println!("Image successfully created."),
        Err(why) => println!("Error: {}", why),
    }
}
