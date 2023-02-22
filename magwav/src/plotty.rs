use std::{f64::consts::PI, ops::Range};

use crate::system::Magnet;
use anyhow as ah;
use nalgebra as na;
use plotters2::{coord::ranged3d::ProjectionMatrix, prelude::*, style::full_palette::BLUEGREY_800};

pub fn plot_system(states: &Vec<na::DMatrix<Magnet>>) -> ah::Result<()> {
    let root = BitMapBackend::gif("plots/testplot.gif", (600, 400), 1000)?.into_drawing_area();

    for magnets in states {
        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .build_cartesian_3d::<Range<f64>, Range<f64>, Range<f64>>(
                -1.0..1.0,
                -1.0..1.0,
                -1.0..1.0,
            )?;

        chart.with_projection(|mut p| {
            p.add_transform(ProjectionMatrix::rotate(-PI * 0.5, 0.0, 0.0));
            p.scale = 0.9;
            p.into_matrix()
        });

        chart
            .configure_axes()
            .light_grid_style(BLACK.mix(0.3))
            .max_light_lines(3)
            .draw()?;

        for (_idx, magnet) in magnets.iter().enumerate() {
            chart
                .draw_series(LineSeries::new(
                    [(0.0, 0.0, 0.0), (magnet.x, magnet.y, magnet.z)].into_iter(),
                    &BLUEGREY_800,
                ))
                .unwrap();
        }

        chart
            .draw_series(LineSeries::new(
                [(0.0, 0.0, 0.0), (1.0, 0.0, 0.0)].into_iter(),
                &GREEN,
            ))
            .unwrap();

        // Red y axis
        chart
            .draw_series(LineSeries::new(
                [(0.0, 0.0, 0.0), (0.0, 1.0, 0.0)].into_iter(),
                &RED,
            ))
            .unwrap();

        root.present()?;
    }

    root.present().expect(
        "Unable to write result to file, please make sure 'plots' dir exists under current dir",
    );
    println!("Result has been saved to testplot.gif");

    Ok(())
}
