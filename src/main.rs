mod algebra;
mod ppm;
mod color;

fn main() {
    ppm::create_blue_gradient_background("../blue.ppm".to_string(), 200, 100);
}
