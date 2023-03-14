use anyhow::{Ok, Result};
use nalgebra as na;
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
