#![allow(unused)]
mod fractal_generator;
mod grid;
mod plotty;

use fractal_generator::generate_fractal_drum;
use grid::Grid;
use ndarray::s;
use plotty::{plot_grid_2d, plot_sln};

fn main() {
    let side_length = 2.0;
    let level = 2;
    let fractal = generate_fractal_drum(side_length, level);
    let griddy = Grid::from_fractal_marked_single(&fractal, side_length, level);
    println!("Grid dimension {:?}", griddy.grid.dim());
    // plot_grid_2d(griddy.grid, "images/devimg.jpg").unwrap();
    // Grid::time_mark_fns(fractal, side_length, level);
    // griddy.solve_sparse();
    let (eigs, sol) = griddy.solve();

    plot_sln(sol.slice(s![0, .., ..]), "images/solution0.jpg").unwrap();
    // plot_sln(sol.slice(s![1, .., ..]), "images/solution1.jpg").unwrap();
    // plot_sln(sol.slice(s![2, .., ..]), "images/solution2.jpg").unwrap();
    // plot_sln(sol.slice(s![-1, .., ..]), "images/solution-1.jpg").unwrap();
}
