use nalgebra as na;

pub const E_Z: na::Vector3<f64> = na::Vector3::new(0.0, 0.0, 1.0);
pub const GYROMAGNETIC_RATIO: f64 = 1.6e-11;
pub const BOHR_MAGNETRON: f64 = 5.8e-5;

pub type Magnet = na::Vector3<f64>;

#[derive(Debug, Clone)]
pub struct MagneticSystem {
    pub magnets: na::DMatrix<Magnet>,
    coupling_constant: f64,
    dampening_constant: f64,
    anisotropy_constant: f64,
    temperature: f64,
    magnetic_field: na::Vector3<f64>,
}

//* NB: periodic boundary condition
impl MagneticSystem {
    pub fn new_toy(
        magnets: na::DMatrix<Magnet>,
        coupling_constant: f64,
        dampening_constant: f64,
    ) -> Self {
        Self {
            magnets,
            coupling_constant,
            dampening_constant,
            anisotropy_constant: 0.3 * coupling_constant,
            temperature: 0.1 * coupling_constant,
            magnetic_field: 0.3 * E_Z,
        }
    }

    pub fn step(&mut self, delta_t: f64) {
        // https://i.imgur.com/6kh5IFC.png

        // First heun step:  yp_next = y_n + delta_t * derivative(t, y_n)
        let deriv_mags_t = self.derivative(&self.magnets, 0.0);
        let magnets_p_next = self.magnets.clone() + deriv_mags_t.map(|elem| elem * delta_t);
        // Second heun step: y_next = y_n + delta_t/2 * ( derivative(t, y_n) + derivative(t, yp_next) )
        let derivatives_sum = deriv_mags_t + self.derivative(&magnets_p_next, 1.0);
    }

    fn derivative(&self, magnets: &na::DMatrix<Magnet>, time: f64) -> na::DMatrix<Magnet> {
        // let consts = -GYROMAGNETIC_RATIO / (1.0 + self.dampening_constant);
        // magnets.map(|magnet| {
        //     let h_eff = -1.0/BOHR_MAGNETRON *
        // })
        todo!()
    }
}

fn heisenberg_hamiltonean(magnets: na::DMatrix<Magnet>, magnetic_const: f64) -> f64 {
    let (x_len, y_len) = magnets.shape();
    let mut res = 0.0;

    // Isotropic part
    for j_x in 0..x_len {
        for j_y in 0..y_len {
            for k_x in 0..x_len {
                if k_x == j_x {
                    continue;
                }
                for k_y in 0..y_len {
                    if k_y == j_y {
                        continue;
                    }
                    res += magnetic_const * magnets[(j_x, j_y)].dot(&magnets[(k_x, k_y)]);
                }
            }
        }
    }

    return res;
}
