use std::collections::VecDeque;

use nalgebra as na;
use ndarray::Array2;
use timeit::{timeit, timeit_loops};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GridPoint {
    Inny,
    Outy,
    Wall,
}

pub struct Grid {
    pub grid: Array2<GridPoint>,
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

        Self { grid }
    }

    pub fn from_fractal_marked(
        fractal: Vec<na::Point2<f64>>,
        side_length: f64,
        level: u32,
        dblres: bool,
    ) -> Self {
        let mut me = Self::from_fractal(&fractal, side_length, level, dblres);
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
}

fn mark_inside_bfs(grid: &mut Array2<GridPoint>) {
    let (mid_y, mid_x): (usize, usize) = (grid.dim().0 / 2, grid.dim().1 / 2);
    let mut queue = VecDeque::new();
    grid[(mid_y, mid_x)] = GridPoint::Inny;
    queue.push_back((mid_y, mid_x));

    while let Some((y, x)) = queue.pop_front() {
        let diffs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dy, dx) in diffs {
            if x == 0 || y == 0 {
                println!("{x}, {y}, {:?}", grid[(y, x)]);
            }
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
            if x == 0 || y == 0 {
                println!("{x}, {y}, {:?}", grid[(y, x)]);
            }
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
    for ((y, _x), pt) in grid.indexed_iter_mut() {
        if y == 0 {
            wall_counter = 0;
            prev = GridPoint::Outy;
        }

        if prev == GridPoint::Wall && *pt != GridPoint::Wall {
            wall_counter += 1;
        }

        if *pt != GridPoint::Wall && wall_counter % 2 == 0 {
            *pt = GridPoint::Inny;
        }

        prev = *pt;
    }
}

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
