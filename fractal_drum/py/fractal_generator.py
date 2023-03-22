def generate_fractal_drum(
    side_length: float, depth: int
) -> tuple[list[float], list[float]]:
    """
    returns xs, ys"""
    side_length = side_length / 2.0
    xs: list = [-side_length, side_length, side_length, -side_length]
    ys: list = [side_length, side_length, -side_length, -side_length]

    for _ in range(depth):
        new_xs = []
        new_ys = []
        for i in range(len(xs) - 1):
            pt1_x = xs[i]
            pt1_y = ys[i]
            pt8_x = xs[i + 1]
            pt8_y = ys[i + 1]
            new_xs.append(pt1_x)
            new_ys.append(pt1_y)
            rest_pts_x, rest_pts_y = generate_from_line(pt1_x, pt1_y, pt8_x, pt8_y)
            new_xs += rest_pts_x
            new_ys += rest_pts_y

        pt1_x = xs[-1]
        pt1_y = ys[-1]
        pt8_x = xs[0]
        pt8_y = ys[0]
        new_xs.append(pt1_x)
        new_ys.append(pt1_y)
        rest_pts_x, rest_pts_y = generate_from_line(pt1_x, pt1_y, pt8_x, pt8_y)
        new_xs += rest_pts_x
        new_ys += rest_pts_y
        xs = new_xs
        ys = new_ys

    xs.append(xs[0])
    ys.append(ys[0])
    return xs, ys


def generate_from_line(
    start_x: float, start_y: float, stop_x: float, stop_y: float
) -> tuple[list[float], list[float]]:
    """returns xs, ys"""
    diff_x = (stop_x - start_x) * 0.25
    diff_y = (stop_y - start_y) * 0.25

    pt2_x = start_x + diff_x
    pt2_y = start_y + diff_y
    pt9_x = pt2_x + diff_x * 2.0
    pt9_y = pt2_y + diff_y * 2.0

    # diff rotated 90 deg ccw
    normal_x = -diff_y
    normal_y = diff_x

    pt3_x = pt2_x + normal_x
    pt3_y = pt2_y + normal_y
    pt4_x = pt3_x + diff_x
    pt4_y = pt3_y + diff_y
    pt5_x = pt4_x - normal_x
    pt5_y = pt4_y - normal_y
    pt6_x = pt5_x - normal_x
    pt6_y = pt5_y - normal_y
    pt7_x = pt6_x + diff_x
    pt7_y = pt6_y + diff_y
    pt8_x = pt7_x + normal_x
    pt8_y = pt7_y + normal_y

    return (
        [pt2_x, pt3_x, pt4_x, pt5_x, pt6_x, pt7_x, pt8_x, pt9_x],
        [pt2_y, pt3_y, pt4_y, pt5_y, pt6_y, pt7_y, pt8_y, pt9_y],
    )
