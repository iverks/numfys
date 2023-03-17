#![allow(unused)]
mod fractal_generator;
mod grid;
mod plotty;

use fractal_generator::generate_fractal_drum;
use grid::Grid;
use plotty::plot_grid_2d;

fn main() {
    let side_length = 2.0;
    let level = 3;
    let fractal = generate_fractal_drum(side_length, level);
    // let griddy = Grid::from_fractal(&fractal, side_length, level, true);
    // plot_grid_2d(griddy.grid, "images/devimg.jpg").unwrap();
    Grid::time_mark_fns(fractal, side_length, level);
}
