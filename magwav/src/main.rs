mod plotty;
mod system;
#[cfg(test)]
mod tests;

use nalgebra as na;
use plotty::plot_system;
use system::{Magnet, MagneticSystem};

fn task_2_1() {
    let magnet = {
        let (mx, my) = (0.2, 0.0);
        Magnet::new(mx, my, (1.0 - mx.powi(2) - my.powi(2)).sqrt())
    };
    let mut sys = MagneticSystem::new_toy(na::DMatrix::from_element(1, 1, magnet), 10.0);
    let mut states = vec![sys.magnets.clone()];

    // for _ in 0..10 {
    //     sys.step();
    //     states.push(sys.magnets.clone());
    // }

    plot_system(&states).unwrap();
}

fn main() {
    println!("Hello, world!");
    task_2_1();
}
