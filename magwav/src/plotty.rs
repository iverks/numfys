use std::{f64::consts::PI, ops::Range};

use crate::system::Magnet;
use anyhow as ah;
use nalgebra as na;
use ndarray::Array3;
use plotters::{
    coord::ranged3d::ProjectionMatrix,
    prelude::*,
    style::full_palette::{GREY, ORANGE_600},
};

pub enum PlotDirection {
    Testy,
    Task2_1_1,
    Task2_1_2,
    Task2_1_3,
    Task2_2_2_1,
    Task2_3_1,
}

pub fn plot_system(
    states: &Vec<Array3<Magnet>>,
    filename: &str,
    frame_delay: u32,
    plot_direction: PlotDirection,
) -> ah::Result<()> {
    let root = BitMapBackend::gif("plots/".to_owned() + filename, (600, 400), frame_delay)?
        .into_drawing_area();
    let (mut minx, mut maxx) = (-1.0, 1.0);
    let (mut miny, mut maxy) = (-1.0, 1.0);
    let (mut minz, mut maxz) = (-1.0, 1.0);

    if let PlotDirection::Task2_3_1 = plot_direction {
        (minx, maxx) = (-10.0, 10.0);
        (miny, maxy) = (-10.0, 10.0);
        (minz, maxz) = (-10.0, 10.0);
    }

    for (frame, magnets) in states.iter().enumerate() {
        root.fill(&WHITE).unwrap();

        let num_magnets = magnets.len();
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
            match plot_direction {
                PlotDirection::Testy => {
                    p.yaw = 0.0;
                    p.pitch = PI * 0.5 - 0.01;
                }
                PlotDirection::Task2_1_3 => {
                    p.yaw = 0.1;
                    p.pitch = PI * 0.5 - 0.01;
                }
                PlotDirection::Task2_2_2_1 => {
                    p.yaw = 0.0;
                    p.pitch = PI * 0.5;
                    // p.pitch = PI * 0.5 - 0.4;
                }
                _ => (),
            }

            p.into_matrix()
        });

        chart
            .configure_axes()
            .light_grid_style(BLACK.mix(0.3))
            .max_light_lines(3)
            .draw()?;

        let (z_range, y_range, x_range) = magnets.dim();
        let (del_x, del_y, del_z) = (
            (maxx - minx) / x_range as f64,
            (maxy - miny) / y_range as f64,
            (maxz - minz) / z_range as f64,
        );
        for z in 0..z_range {
            for y in 0..y_range {
                for x in 0..x_range {
                    let magnet = magnets[(z, y, x)];
                    let (x, y, z) = (
                        minx + (x as f64 + 0.5) * del_x,
                        miny + (y as f64 + 0.5) * del_y,
                        minz + (z as f64 + 0.5) * del_z,
                    );

                    let line = if num_magnets < 10 {
                        vec![(x, y, z), (x + magnet.x, y + magnet.y, z + magnet.z)].into_iter()
                    } else {
                        vec![
                            (x, y, z),
                            (x + magnet.x, y + magnet.y, z + magnet.z),
                            (x + magnet.x, y + magnet.y, z),
                        ]
                        .into_iter()
                    };

                    let mut pts = vec![(x, y, z), (x + magnet.x, y + magnet.y, z + magnet.z)];
                    match plot_direction {
                        PlotDirection::Testy => pts.push((x + magnet.x, y + magnet.y, z)),
                        _ => (),
                    }
                    chart
                        .draw_series(LineSeries::new(pts.into_iter(), &ORANGE_600))
                        .unwrap();
                }
            }
        }

        // if plot_direction is UpAndDown
        match plot_direction {
            PlotDirection::Task2_1_1 | PlotDirection::Task2_1_2 => {
                chart.draw_series(
                    SurfaceSeries::xoy(
                        (-30..30).map(|n| n as f64 / 30.0 * (maxx - minx) / 2.0),
                        (-30..30).map(|n| n as f64 / 30.0 * (maxy - miny) / 2.0),
                        |_, _| 0.0,
                    )
                    .style(&BLACK.mix(0.2)),
                );
            }
            _ => (),
        }

        root.present()?;
    }

    root.present().expect(
        "Unable to write result to file, please make sure 'plots' dir exists under current dir",
    );
    println!("Result has been saved to {filename}");

    Ok(())
}
