use std::{f64::consts::PI, ops::Range};

use crate::system::Magnet;
use anyhow as ah;
use nalgebra as na;
use plotters::{
    coord::ranged3d::ProjectionMatrix,
    prelude::*,
    style::full_palette::{GREY, ORANGE_600},
};

pub fn plot_system(
    states: &Vec<na::DMatrix<Magnet>>,
    filename: &str,
    frame_delay: u32,
) -> ah::Result<()> {
    let root = BitMapBackend::gif("plots/".to_owned() + filename, (600, 400), frame_delay)?
        .into_drawing_area();
    let (minx, maxx) = (-1.0, 1.0);
    let (miny, maxy) = (-1.0, 1.0);
    let (minz, maxz) = (0.0, 1.0);

    for (frame, magnets) in states.iter().enumerate() {
        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .caption(format!("{frame}"), &GREY)
            .build_cartesian_3d::<Range<f64>, Range<f64>, Range<f64>>(
                minx..maxx,
                miny..maxy,
                minz..maxz,
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

        let (y_range, x_range) = magnets.shape();
        let (del_x, del_y) = (
            (maxx - minx) / x_range as f64,
            (maxy - miny) / y_range as f64,
        );
        for y in 0..y_range {
            for x in 0..x_range {
                let magnet = magnets[(y, x)];
                let (x, y, z) = (
                    minx + (x as f64 + 0.5) * del_x,
                    miny + (y as f64 + 0.5) * del_y,
                    0.0,
                );
                chart
                    .draw_series(
                        LineSeries::new(
                            [
                                (x, y, z),
                                (x + magnet.x, y + magnet.y, z + magnet.z),
                                (x + magnet.x, y + magnet.y, z),
                            ]
                            .into_iter(),
                            &ORANGE_600,
                        )
                        .point_size(3),
                    )
                    .unwrap();
            }
        }

        root.present()?;
    }

    root.present().expect(
        "Unable to write result to file, please make sure 'plots' dir exists under current dir",
    );
    println!("Result has been saved to testplot.gif");

    Ok(())
}
