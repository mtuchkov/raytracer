pub(crate) mod vec;
pub(crate) mod renderer;
pub(crate) mod color;
pub(crate) mod surfaces;
mod camera;
mod ffi;

fn main() {
    renderer::create_image("../blue.ppm".to_string(), 200, 100);
}
