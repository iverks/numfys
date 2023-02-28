// #![allow(unused)]
use nalgebra as na;
use rand::thread_rng;
use rand_distr::{Distribution, Normal};

pub const E_Z: na::Vector3<f64> = na::Vector3::new(0.0, 0.0, 1.0);
pub const GYROMAGNETIC_RATIO: f64 = 1.6e-11;
pub const BOHR_MAGNETRON: f64 = 5.8e-5;

pub type Magnet = na::Vector3<f64>;

#[derive(Debug, Clone)]
pub struct MagneticSystem {
    pub magnets: na::DMatrix<Magnet>,
    pub dampening_constant: f64,
    pub coupling_constant: f64,
    pub anisotropy_constant: f64,
    pub temperature: f64,
    pub magnetic_field: na::Vector3<f64>,
    pub timestep: f64,
}

//* NB: periodic boundary condition
impl MagneticSystem {
    pub fn new_toy(
        magnets: na::DMatrix<Magnet>,
        coupling_constant: f64,
        dampening_constant: f64,
        timestep: f64,
    ) -> Self {
        Self {
            magnets,
            dampening_constant,
            coupling_constant,
            anisotropy_constant: 0.3 * coupling_constant,
            temperature: 0.1 * coupling_constant,
            magnetic_field: 0.3 * coupling_constant * E_Z,
            timestep,
        }
    }

    pub fn step(&mut self) {
        // First heun step:  yp_next = y_n + delta_t * derivative(t, y_n)
        let deriv_mags_t = self.derivative(&self.magnets);
        let magnets_p_next = self.magnets.clone() + deriv_mags_t.map(|elem| elem * self.timestep);
        // Second heun step: y_next = y_n + delta_t/2 * ( derivative(t, y_n) + derivative(t, yp_next) )
        let derivatives_sum = deriv_mags_t + self.derivative(&magnets_p_next);

        self.magnets += derivatives_sum.map(|elem| elem * self.timestep / 2.0);

        // Normalize all magnets
        for mut magnet in self.magnets.iter_mut() {
            magnet.normalize_mut();
        }
    }

    fn derivative(&self, magnets: &na::DMatrix<Magnet>) -> na::DMatrix<Magnet> {
        let (y_range, x_range) = magnets.shape();
        let mut derivatives = magnets.clone();
        for y in 0..y_range {
            for x in 0..x_range {
                // Find h_eff for given magnet
                // Sum over nearest neighbours to find coupling term
                let mut nearest_sum = Magnet::new(0.0, 0.0, 0.0);
                for (dx, dy) in [(-1 as i32, 0 as i32), (1, 0), (0, -1), (0, 1)] {
                    let new_y = ((y as i32 + dy) as usize) % y_range;
                    let new_x = ((x as i32 + dx) as usize) % x_range;
                    nearest_sum += magnets[(new_y, new_x)];
                }

                let coupling = nearest_sum * self.coupling_constant / 2.0;

                // Find anisotropy term
                let cur_mag = magnets[(y, x)];
                let anisotropy = 2.0 * (cur_mag.dot(&E_Z)) * E_Z;

                // Find siemen term
                let siemen = self.magnetic_field;

                // Find noise term
                let noise_term = self.random_noise_magnet();

                // Note that there is a double negative so 1.0 is positive
                let h_eff = 1.0 / BOHR_MAGNETRON * (coupling + anisotropy + siemen) + noise_term;

                // Find actual derivative
                let magnet_cross_h = magnets[(y, x)].cross(&h_eff);
                let derivative = -GYROMAGNETIC_RATIO / (1.0 + self.dampening_constant)
                    * (magnet_cross_h
                        + self.dampening_constant * magnets[(y, x)].cross(&magnet_cross_h));

                derivatives[(y, x)] = derivative;
            }
        }
        derivatives
    }

    fn random_noise_magnet(&self) -> Magnet {
        if self.coupling_constant == 0.0 {
            return Magnet::new(0.0, 0.0, 0.0);
        }
        let mut rng = thread_rng();
        let mut normal = Normal::new(0.0, 1.0).unwrap();
        let mut magnet = Magnet::from_fn(|_, _| normal.sample(&mut rng));

        let consts = 2.0 * self.dampening_constant * self.temperature
            / (self.coupling_constant * BOHR_MAGNETRON * self.timestep);

        println!("consts {consts}");

        magnet *= consts.sqrt();
        magnet
    }
}
