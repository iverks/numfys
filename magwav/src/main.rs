#![allow(unused)]

mod plotty;
mod system;
#[cfg(test)]
mod tests;

use std::time::{self, Instant};

use nalgebra as na;
use ndarray::{s, Array3};
use plotty::{plot_system, PlotDirection};
use rand::{thread_rng, Rng};
use rand_distr::uniform;
use system::{Magnet, MagneticSystem, E_Z};

fn testy() {
    let straight = {
        let (mx, my) = (0.0, 0.0);
        Magnet::new(mx, my, (1.0 - mx.powi(2) - my.powi(2)).sqrt())
    };

    let tilted = {
        let (mx, my) = (-0.2, 0.0);
        Magnet::new(mx, my, (1.0 - mx.powi(2) - my.powi(2)).sqrt())
    };

    let tilted2 = {
        let (mx, my) = (0.2, 0.0);
        Magnet::new(mx, my, (1.0 - mx.powi(2) - my.powi(2)).sqrt())
    };

    let mut sys = MagneticSystem {
        magnets: Array3::from_elem((1, 1, 10), straight),
        dampening_constant: 0.0,
        coupling_constant: 0.000,
        anisotropy_constant: 100e-3,
        temperature: 0.0e-4,
        magnetic_field: 0.0 * E_Z,
        timestep: 5e-16,
    };

    sys.magnets[(0, 0, 5)] = tilted;
    sys.magnets[(0, 0, 4)] = tilted2;

    let mut states = vec![sys.magnets.clone()];

    for _ in 0..200 {
        for _ in 0..1 {
            sys.step();
        }
        states.push(sys.magnets.clone());
    }

    plot_system(&states, "testplot.gif", 100, PlotDirection::Testy).unwrap();
}

fn task_2_1_1() {
    let start = Instant::now();
    let magnet = {
        let (mx, my) = (0.3, 0.3);
        Magnet::new(mx, my, (1.0 - mx.powi(2) - my.powi(2)).sqrt())
    };

    // let mut sys = MagneticSystem::new_toy(Array3::from_element(1, 1, magnet), 10e-3, 0.0, 1e6);
    let mut sys = MagneticSystem {
        magnets: Array3::from_elem((1, 1, 1), magnet),
        dampening_constant: 0.0,
        coupling_constant: 0.1,
        anisotropy_constant: 1e-3,
        temperature: 0.0e-4,
        magnetic_field: 0.0 * E_Z,
        timestep: 1e-15,
    };
    let mut states = vec![sys.magnets.clone()];

    for _ in 0..200 {
        for _ in 0..1 {
            sys.step();
        }
        states.push(sys.magnets.clone());
        // dbg!(&states);
    }

    println!("Simulation took {} ms", start.elapsed().as_millis());

    let start = Instant::now();
    plot_system(&states, "testplot.gif", 100, PlotDirection::Task2_1_1).unwrap();
    println!("Plotting took {} s", start.elapsed().as_secs());
}

fn task_2_1_2() {
    let start = Instant::now();

    let magnets = {
        let mut rng = thread_rng();
        // let mut normal = Normal::new(0.0, 1.0).unwrap();
        Array3::from_shape_fn((1, 1, 10), |_| {
            let magnet = Magnet::from_fn(|_, _| rng.gen_range(-1.0..1.0));
            magnet.normalize()
        })
    };

    // let mut sys = MagneticSystem::new_toy(Array3::from_element(1, 1, magnet), 10e-3, 0.0, 1e6);
    let mut sys = MagneticSystem {
        magnets,
        dampening_constant: 0.5,
        coupling_constant: 10e-3,
        anisotropy_constant: 1e-3,
        temperature: 0.0,
        magnetic_field: 0.0 * E_Z,
        timestep: 0.1e-15,
    };
    let mut states = vec![sys.magnets.clone()];

    for _ in 0..200 {
        for _ in 0..50 {
            sys.step();
        }
        states.push(sys.magnets.clone());
        // dbg!(&states);
    }
    println!("Simulation took {} s", start.elapsed().as_secs());

    plot_system(&states, "testplot.gif", 100, PlotDirection::Task2_1_2).unwrap();
}

fn task_2_1_3() {
    let start = Instant::now();

    let mut magnets = Array3::from_elem((1, 1, 100), Magnet::new(0.0, 0.0, 1.0));

    // Tilt the central magnet(s)
    magnets[(0, 0, 50)] = Magnet::new(0.9, 0.0, 1.0).normalize();

    // let mut sys = MagneticSystem::new_toy(Array3::from_element(1, 1, magnet), 10e-3, 0.0, 1e6);
    let mut sys = MagneticSystem {
        magnets,
        dampening_constant: 0.0,
        coupling_constant: 10.0 * 1e-3,
        anisotropy_constant: 3.0 * 1e-3,
        temperature: 0.0,
        magnetic_field: 0.0 * E_Z,
        timestep: 1e-15,
    };
    let mut states = vec![sys.magnets.clone()];

    for _ in 0..400 {
        for _ in 0..4 {
            sys.step();
        }
        states.push(sys.magnets.clone());
        // dbg!(&states);
    }

    println!("Simulation took {} ms", start.elapsed().as_millis());

    let start = Instant::now();
    plot_system(&states, "testplot.gif", 100, PlotDirection::Task2_1_3).unwrap();
    println!("Simulation took {} ms", start.elapsed().as_millis());
}

fn task_2_2_2() {
    let start = Instant::now();

    let mut magnets = Array3::from_elem((1, 1, 50), Magnet::new(0.0, 0.0, 1.0));

    let mut sys = MagneticSystem {
        magnets,
        dampening_constant: 0.01,
        coupling_constant: 10.0 * 1e-3,
        anisotropy_constant: 3e-3,
        temperature: 0.5 * 1e-3,
        magnetic_field: 0.0 * E_Z,
        timestep: 0.5 * 1e-15,
    };
    let mut states = vec![sys.magnets.clone()];
    let mut x_components: Vec<Vec<f64>> = vec![sys.magnets.iter().map(|mag| mag.x).collect()];

    // 60 000 * 0.5 fs = 30 ps
    for _ in 0..60000 {
        for _ in 0..1 {
            sys.step();
            x_components.push(sys.magnets.iter().map(|mag| mag.x).collect());
        }
        states.push(sys.magnets.clone());
        // dbg!(&states);
    }

    plot_system(&states, "testplot.gif", 100, PlotDirection::Task2_2_2_1).unwrap();
    std::fs::write(
        "plots/x_components.json",
        serde_json::to_string_pretty(&x_components).expect("Cant jsonify"),
    )
    .expect("cant write json to file");
}

fn main() {
    // testy();
    // task_2_1_1();
    // task_2_1_2();
    // task_2_1_3();
    task_2_2_2();
}
