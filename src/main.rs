mod algebra;
mod renderer;
mod color;

fn main() {
    renderer::create_image("../blue.ppm".to_string(), 200, 100);
}
