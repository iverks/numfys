use std::collections::VecDeque;

use eigenvalues::{
    davidson::Davidson, lanczos::HermitianLanczos, utils::sort_eigenpairs, DavidsonCorrection,
    SpectrumTarget,
};
use lapack::dgees;
use na::{ComplexField, DMatrix};
use nalgebra as na;
use nalgebra_sparse::{CooMatrix, CsrMatrix};
use ndarray::{s, Array, Array1, Array2, Array3, AssignElem, ShapeBuilder};
use ndarray_linalg::{eig, Eig, Eigh, EighInplace, UPLO};
use sprs::{assign_to_dense, CsMat, TriMat};
use sprs_ldl::{Ldl, LdlNumeric};
use timeit::{timeit, timeit_loops};

use crate::plotty;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GridPoint {
    Inny,
    Outy,
    Wall,
}

pub struct Grid {
    pub grid: Array2<GridPoint>,
    pub grid_const: f64,
}

impl Grid {
    pub fn from_fractal(
        fractal: &Vec<na::Point2<f64>>,
        side_length: f64,
        level: u32,
        dblres: bool,
    ) -> Self {
        let grid_const = side_length / 4_i32.pow(level) as f64;
        let grid_const = if dblres { grid_const / 2.0 } else { grid_const };
        let max_x = fractal.iter().max_by(|a, b| a.x.total_cmp(&b.x)).unwrap().x;
        let min_x = fractal.iter().min_by(|a, b| a.x.total_cmp(&b.x)).unwrap().x;
        let max_y = fractal.iter().max_by(|a, b| a.y.total_cmp(&b.y)).unwrap().y;
        let min_y = fractal.iter().min_by(|a, b| a.y.total_cmp(&b.y)).unwrap().y;

        let shape_x = ((max_x - min_x) / grid_const).round() as usize + 1;
        let shape_y = ((max_y - min_y) / grid_const).round() as usize + 1;

        let mut grid: Array2<GridPoint> = Array2::from_elem((shape_y, shape_x), GridPoint::Outy);

        if dblres {
            for pts in fractal.windows(2) {
                let (pt1, pt2) = (pts[0], pts[1]);
                let diff = pt2 - pt1;
                let midpt = pt1 + (diff / 2.0);
                for pt in [pt1, midpt] {
                    let new_y = ((pt.y - min_y) / grid_const) as usize;
                    let new_x = ((pt.x - min_x) / grid_const) as usize;
                    grid[(new_y, new_x)] = GridPoint::Wall;
                }
            }
        } else {
            for pt in fractal {
                let new_y = ((pt.y - min_y) / grid_const) as usize;
                let new_x = ((pt.x - min_x) / grid_const) as usize;
                grid[(new_y, new_x)] = GridPoint::Wall;
            }
        }

        Self { grid, grid_const }
    }

    pub fn from_fractal_marked_double(
        fractal: &Vec<na::Point2<f64>>,
        side_length: f64,
        level: u32,
    ) -> Self {
        let mut me = Self::from_fractal(&fractal, side_length, level, true);
        // mark_inside_line_trick(&mut me.grid);
        mark_inside_bfs(&mut me.grid);
        me
    }

    pub fn from_fractal_marked_single(
        fractal: &Vec<na::Point2<f64>>,
        side_length: f64,
        level: u32,
    ) -> Self {
        let mut me = Self::from_fractal(&fractal, side_length, level, false);
        mark_inside_bfs(&mut me.grid);
        me
    }

    pub fn time_mark_fns(fractal: Vec<na::Point2<f64>>, side_length: f64, level: u32) {
        let griddy = Self::from_fractal(&fractal, side_length, level, false);
        let griddy_dbl = Self::from_fractal(&fractal, side_length, level, true);

        let num_loops = 10;
        println!("Level {level}");
        print!("bfs single resolution | ");
        timeit!({ mark_inside_bfs(&mut griddy.grid.clone()) });

        print!("bfs double resolution | ");
        timeit!({ mark_inside_bfs(&mut griddy_dbl.grid.clone()) });

        print!("dfs single resolution | ");
        timeit!({ mark_inside_dfs(&mut griddy.grid.clone()) });

        print!("dfs double resolution | ");
        timeit!({ mark_inside_dfs(&mut griddy_dbl.grid.clone()) });

        print!("lt double resolution | ");
        timeit!({ mark_inside_line_trick(&mut griddy_dbl.grid.clone()) });

        print!("lt_slow double resolution | ");
        timeit!({ mark_inside_line_trick_slower(&mut griddy_dbl.grid.clone()) });
    }

    pub fn solve_sparse_nalgebra(&self) {
        let n = self.grid.dim().0;
        let mut eq = CooMatrix::new(n * n, n * n);

        let grid_const_squared = self.grid_const.powi(2);
        for i in 0..n * n {
            let y = i / n;
            let x = i % n;

            // Skip points outside
            match self.grid[(y, x)] {
                GridPoint::Outy | GridPoint::Wall => {
                    continue;
                }
                _ => (),
            }
            eq.push(i, y * n + x, 4.0 / grid_const_squared);

            for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let new_y = (y as i32 + dy) as usize;
                let new_x = (x as i32 + dx) as usize;
                if new_x >= n || new_y >= n {
                    continue;
                }
                eq.push(i, new_y * n + new_x, -1.0 / grid_const_squared);
            }
        }

        let eq_to_solve = CsrMatrix::from(&eq);
        // let dense = na::DMatrix::from(eq_to_solve.to_owned());
    }

    pub fn solve_sparse(&self) -> (Array1<f64>, Array3<f64>) {
        let n = self.grid.dim().0;
        let mut eq = TriMat::new((n * n, n * n));

        let grid_const_squared = self.grid_const.powi(2);
        for i in 0..n * n {
            let y = i / n;
            let x = i % n;

            // Skip points outside
            match self.grid[(y, x)] {
                GridPoint::Outy | GridPoint::Wall => {
                    continue;
                }
                _ => (),
            }
            eq.add_triplet(i, y * n + x, 4.0 / grid_const_squared);

            for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let new_y = (y as i32 + dy) as usize;
                let new_x = (x as i32 + dx) as usize;
                if new_x >= n || new_y >= n {
                    continue;
                }
                eq.add_triplet(i, new_y * n + new_x, -1.0 / grid_const_squared);
            }
        }

        let csmat: CsMat<_> = eq.to_csc();

        let dense = csmat.to_dense();
        let (eig, eigvec) = dense.eigh(UPLO::Lower).unwrap();

        let mut vecs_reshaped = Array3::<f64>::zeros((n * n, n, n));
        for ((eiglevel, i), energy) in eigvec.indexed_iter() {
            let y = i / n;
            let x = i % n;
            vecs_reshaped[(eiglevel, y, x)] = *energy;
        }

        (eig, vecs_reshaped)

        // HOW TO SOLVE https://github.com/sparsemat/sprs
    }

    pub fn solve(&self) -> (Array1<f64>, Array3<f64>) {
        let n = self.grid.dim().0;
        let mut energies = Array2::<f64>::zeros((n * n, n * n));

        let grid_const_squared = self.grid_const.powi(2);
        for i in 0..n * n {
            let y = i / n;
            let x = i % n;

            // Skip points outside
            match self.grid[(y, x)] {
                GridPoint::Outy | GridPoint::Wall => {
                    continue;
                }
                _ => (),
            }

            energies[(i, i)] = -4.0 / grid_const_squared;

            for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let new_y = (y as i32 + dy) as usize;
                let new_x = (x as i32 + dx) as usize;
                if new_x >= n || new_y >= n {
                    continue;
                }
                energies[(i, new_y * n + new_x)] = 1.0 / grid_const_squared;
            }
        }

        println!("solving");

        let (eigs, vecs) = energies.eigh(UPLO::Lower).unwrap();

        println!("SOLVED, {} eigs are {eigs}", eigs.len());

        let mut vecs_reshaped = Array3::<f64>::zeros((n * n, n, n));
        for ((eiglevel, i), energy) in vecs.indexed_iter() {
            let y = i / n;
            let x = i % n;
            vecs_reshaped[(eiglevel, y, x)] = *energy;
        }

        (eigs, vecs_reshaped)
    }
}

fn mark_inside_bfs(grid: &mut Array2<GridPoint>) {
    let (mid_y, mid_x): (usize, usize) = (grid.dim().0 / 2, grid.dim().1 / 2);
    let mut queue = VecDeque::new();
    grid[(mid_y, mid_x)] = GridPoint::Inny;
    queue.push_back((mid_y, mid_x));

    while let Some((y, x)) = queue.pop_front() {
        let diffs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dy, dx) in diffs {
            let (new_y, new_x) = ((y as i32 + dy) as usize, (x as i32 + dx) as usize);
            match grid[(new_y, new_x)] {
                GridPoint::Outy => {
                    grid[(new_y, new_x)] = GridPoint::Inny;
                    queue.push_back((new_y, new_x))
                }
                GridPoint::Wall => {
                    continue;
                }
                GridPoint::Inny => {
                    continue;
                }
            }
        }
    }
}

fn mark_inside_dfs(grid: &mut Array2<GridPoint>) {
    let (mid_y, mid_x): (usize, usize) = (grid.dim().0 / 2, grid.dim().1 / 2);
    let mut queue = VecDeque::new();
    grid[(mid_y, mid_x)] = GridPoint::Inny;
    queue.push_back((mid_y, mid_x));
    while let Some((y, x)) = queue.pop_back() {
        let diffs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dy, dx) in diffs {
            let (new_y, new_x) = ((y as i32 + dy) as usize, (x as i32 + dx) as usize);
            match grid[(new_y, new_x)] {
                GridPoint::Outy => {
                    grid[(new_y, new_x)] = GridPoint::Inny;
                    queue.push_back((new_y, new_x))
                }
                GridPoint::Wall => {
                    continue;
                }
                GridPoint::Inny => {
                    continue;
                }
            }
        }
    }
}

fn mark_inside_line_trick(grid: &mut Array2<GridPoint>) {
    let mut wall_counter = 0;
    let mut prev = GridPoint::Outy;
    for ((y, x), pt) in grid.indexed_iter_mut() {
        if x == 0 {
            wall_counter = 0;
            prev = GridPoint::Outy;
        }

        if prev == GridPoint::Wall && *pt != GridPoint::Wall {
            wall_counter += 1;
        }

        if *pt != GridPoint::Wall && wall_counter % 2 != 0 {
            *pt = GridPoint::Inny;
        }

        prev = *pt;
    }
}

/// Note: is bugged
fn mark_inside_line_trick_slower(grid: &mut Array2<GridPoint>) {
    for y in 0..grid.dim().0 {
        for x in 0..grid.dim().1 {
            if grid[(y, x)] == GridPoint::Wall {
                continue;
            }
            let mut wall_counter = 0;
            let mut prev = GridPoint::Outy;
            for new_y in 0..=y {
                if prev == GridPoint::Wall && grid[(new_y, x)] != GridPoint::Wall {
                    wall_counter += 1;
                }
            }

            if wall_counter % 2 == 0 {
                grid[(y, x)] = GridPoint::Inny;
            }
        }
    }
}
