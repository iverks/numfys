#![allow(unused)]

mod plotty;
mod system;
#[cfg(test)]
mod tests;

use nalgebra as na;
use plotty::plot_system;
use system::{Magnet, MagneticSystem};

fn task_2_1() {
    let magnet = {
        let (mx, my) = (0.5, 0.2);
        Magnet::new(mx, my, (1.0 - mx.powi(2) - my.powi(2)).sqrt())
    };

    let mut sys = MagneticSystem::new_toy(na::DMatrix::from_element(1, 1, magnet), 10.0, 0.0, 1e4);
    // let mut sys = MagneticSystem::new_toy(na::DMatrix::from_element(2, 2, magnet), 10.0, 1.0, 1e4);
    let mut states = vec![sys.magnets.clone()];

    for _ in 0..100 {
        for _ in 0..5 {
            sys.step();
        }
        states.push(sys.magnets.clone());
        // dbg!(&states);
    }

    plot_system(&states, "testplot.gif", 100).unwrap();
}

fn main() {
    task_2_1();
}
