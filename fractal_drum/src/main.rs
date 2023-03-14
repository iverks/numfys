mod fractal_generator;
mod plotty;

use fractal_generator::generate_fractal_drum;

fn main() {
    let vertices = generate_fractal_drum(0.9, 3);
}
