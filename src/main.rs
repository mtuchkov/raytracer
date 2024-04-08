use crate::scene::builder::SceneBuilder;

// LEARN:
// The module structure is very similar to Node.js.
// It is a DAG of modules where the root is the main.rs file.
// Because compiler enforces the DAG structure, there are no circular dependencies,
// and there is no need for #ifndev/#define like in C++.
//
// What's even more important the IDE can easily navigate through the modules
// and highlight the errors following the language specification.
mod vec;
mod renderer;
mod color;
mod surfaces;
mod camera;
mod ffi;
mod material;
mod scene;

fn main() {
    let scene_builder = scene::builder::BuiltIn::random();
    let scene = scene_builder.build();
    renderer::create_image(&scene, "../blue.ppm".to_string());
}
