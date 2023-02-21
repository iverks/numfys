use nalgebra as na;

pub const E_Z: na::Vector3<f64> = na::Vector3::new(0.0, 0.0, 1.0);
pub const GYROMAGNETIC_RATIO: f64 = 1.6e-11;

pub type Magnet = na::Vector3<f64>;

#[derive(Debug, Clone)]
pub struct MagneticSystem {
    pub magnets: na::DMatrix<Magnet>,
    coupling_constant: f64,
    anisotropy_constant: f64,
    temperature: f64,
    magnetic_field: na::Vector3<f64>,
}

//* NB: periodic boundary condition
impl MagneticSystem {
    pub fn new_toy(magnets: na::DMatrix<Magnet>, coupling_constant: f64) -> Self {
        Self {
            magnets,
            coupling_constant,
            anisotropy_constant: 0.3 * coupling_constant,
            temperature: 0.1 * coupling_constant,
            magnetic_field: 0.3 * E_Z,
        }
    }

    pub fn step(&mut self) {
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
