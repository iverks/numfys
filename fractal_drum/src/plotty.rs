use crate::grid::GridPoint;
use anyhow::{Ok, Result};
use nalgebra as na;
use ndarray::Array2;
use plotters::prelude::*;

pub fn plot_fractal_2d(fractal: Vec<na::Point2<f64>>, filename: &str) -> Result<()> {
    let root = BitMapBackend::new(filename, (1024, 768)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Fractal", ("sans-serif", 50))
        .build_cartesian_2d(-2.0f64..2.0f64, -1.5f64..1.5f64)?;

    let vertices: Vec<(f64, f64)> = fractal.iter().map(|pt| (pt.x, pt.y)).collect();
    // chart.draw_series(std::iter::once(Polygon::new(vertices, &RED.mix(0.2))))?;
    chart
        .draw_series(LineSeries::new(vertices.into_iter(), &RED))
        .unwrap();

    root.present().expect(
        "Unable to write result to file, please make sure 'plots' dir exists under current dir",
    );

    Ok(())
}

pub fn plot_grid_2d(grid: Array2<GridPoint>, filename: &str) -> Result<()> {
    let root = BitMapBackend::new(filename, (1024, 768)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Fractal", ("sans-serif", 50))
        .margin(20)
        .x_label_area_size(10)
        .y_label_area_size(10)
        .build_cartesian_2d(0..grid.dim().0, 0..grid.dim().1)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    let plotting_area = chart.plotting_area();

    let range = plotting_area.get_pixel_range();

    let (y_len, x_len) = grid.dim();
    for ((y, x), point) in grid.indexed_iter() {
        let color = match point {
            GridPoint::Inny => &GREEN,
            GridPoint::Outy => continue,
            GridPoint::Wall => &BLACK,
        };
        plotting_area.draw(&Rectangle::new(
            [(x, y), (x + 1, y + 1)],
            Into::<ShapeStyle>::into(color).filled(),
        ))?;
    }

    root.present().expect("Could not write to file :(");

    Ok(())
}
