#![allow(unused)]

mod plotty;
mod system;
#[cfg(test)]
mod tests;

use nalgebra as na;
use plotty::{plot_system, PlotDirection};
use rand::{thread_rng, Rng};
use rand_distr::uniform;
use system::{Magnet, MagneticSystem, E_Z};

fn task_2_1_1() {
    let magnet = {
        let (mx, my) = (0.5, 0.2);
        Magnet::new(mx, my, (1.0 - mx.powi(2) - my.powi(2)).sqrt())
    };

    // let mut sys = MagneticSystem::new_toy(na::DMatrix::from_element(1, 1, magnet), 10e-3, 0.0, 1e6);
    let mut sys = MagneticSystem {
        magnets: na::DMatrix::from_element(1, 1, magnet),
        dampening_constant: 0.1,
        coupling_constant: 0.0,
        anisotropy_constant: 1e-3,
        temperature: 5e-4,
        magnetic_field: 0.0 * E_Z,
        timestep: 5e4,
    };
    let mut states = vec![sys.magnets.clone()];

    for _ in 0..200 {
        for _ in 0..5 {
            sys.step();
        }
        states.push(sys.magnets.clone());
        // dbg!(&states);
    }

    plot_system(&states, "testplot.gif", 100, PlotDirection::Task2_1_1).unwrap();
}

fn task_2_1_2() {
    let magnets = {
        let mut rng = thread_rng();
        // let mut normal = Normal::new(0.0, 1.0).unwrap();
        na::DMatrix::from_fn(1, 10, |_, _| {
            let magnet = Magnet::from_fn(|_, _| rng.gen_range(-1.0..1.0));
            magnet.normalize()
        })
    };

    // let mut sys = MagneticSystem::new_toy(na::DMatrix::from_element(1, 1, magnet), 10e-3, 0.0, 1e6);
    let mut sys = MagneticSystem {
        magnets,
        dampening_constant: 0.5,
        coupling_constant: -10e-3,
        anisotropy_constant: 1e-3,
        temperature: 0.0,
        magnetic_field: 0.0 * E_Z,
        timestep: 1e4,
    };
    let mut states = vec![sys.magnets.clone()];

    for _ in 0..200 {
        for _ in 0..50 {
            sys.step();
        }
        states.push(sys.magnets.clone());
        // dbg!(&states);
    }

    plot_system(&states, "testplot.gif", 100, PlotDirection::Task2_1_2).unwrap();
}

fn task_2_1_3() {
    let mut magnets = na::DMatrix::from_element(1, 100, Magnet::new(0.0, 0.0, 1.0));

    // Tilt the central magnet(s)
    // magnets[(1, 49)] = Magnet::new(1.0, 0.0, 1.0).normalize();
    magnets[(0, 50)] = Magnet::new(1.0, 0.0, 1.0).normalize();

    // let mut sys = MagneticSystem::new_toy(na::DMatrix::from_element(1, 1, magnet), 10e-3, 0.0, 1e6);
    let mut sys = MagneticSystem {
        magnets,
        dampening_constant: 0.0,
        coupling_constant: 10.0 * 10e-3,
        anisotropy_constant: 1e-3,
        temperature: 0.0,
        magnetic_field: 0.0 * E_Z,
        timestep: 5e4,
    };
    let mut states = vec![sys.magnets.clone().columns(40, 20).into()];

    for _ in 0..200 {
        for _ in 0..5 {
            sys.step();
        }
        states.push(sys.magnets.clone().columns(40, 20).into());
        // dbg!(&states);
    }

    println!("Im gonna plot");
    plot_system(&states, "testplot.gif", 100, PlotDirection::Task2_1_3).unwrap();
}

fn main() {
    task_2_1_3();
}
