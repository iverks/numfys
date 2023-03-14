use nalgebra as na;

pub struct Fractal {
    points: Vec<na::Point2<f64>>,
}

pub fn generate_fractal_drum(side_length: f64, depth: i32) -> Vec<na::Point2<f64>> {
    let mut lines: Vec<na::Point2<f64>> = vec![
        // Clockwise from top left
        // top left
        na::Point2::new(-side_length, side_length),
        // top right
        na::Point2::new(side_length, side_length),
        // bottom right
        na::Point2::new(side_length, -side_length),
        // bottom left
        na::Point2::new(-side_length, -side_length),
    ];

    for _ in 0..depth {
        let mut new_lines = vec![];

        for prev_pts in lines.windows(2) {
            let pt1 = prev_pts[0].to_owned();
            let pt8 = prev_pts[1].to_owned();
            new_lines.push(pt1);
            new_lines.append(&mut generate_from_line(pt1, pt8));
        }
        let pt1 = lines[lines.len() - 1].to_owned();
        let pt8 = lines[0].to_owned();
        new_lines.push(pt1);
        new_lines.append(&mut generate_from_line(pt1, pt8));
        lines = new_lines;
    }
    lines.push(lines[0].clone());
    lines
}

pub fn generate_from_line(start: na::Point2<f64>, stop: na::Point2<f64>) -> Vec<na::Point2<f64>> {
    let diff = (stop - start) * 0.25;

    let pt2 = start + diff;
    let pt9 = pt2 + diff * 2.0;

    // diff rotated 90 deg ccw
    let normal = na::Vector2::new(-diff.y, diff.x);

    let pt3 = pt2 + normal;
    let pt4 = pt3 + diff;
    let pt5 = pt4 - normal;
    let pt6 = pt5 - normal;
    let pt7 = pt6 + diff;
    let pt8 = pt7 + normal;

    vec![pt2, pt3, pt4, pt5, pt6, pt7, pt8, pt9]
}
