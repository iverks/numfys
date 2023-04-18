"""
0 -> outside \n
1 -> wall \n
2 -> inside \n
"""
import numpy as np
from collections import deque


def grid_from_fractal(
    frac_xs: list[float],
    frac_ys: list[float],
    side_length: float,
    level: int,
    dblres: bool,
) -> tuple[float, np.ndarray]:

    grid_const = side_length / 4**level
    if dblres:
        grid_const *= 0.5

    min_x = min(frac_xs)
    max_x = max(frac_xs)
    min_y = min(frac_ys)
    max_y = max(frac_ys)

    shape_x = int((max_x - min_x) / grid_const) + 1
    shape_y = int((max_y - min_y) / grid_const) + 1

    grid = np.zeros((shape_y, shape_x))

    if dblres:
        for i in range(len(frac_xs) - 1):
            pt1_x = frac_xs[i]
            pt1_y = frac_ys[i]
            pt2_x = frac_xs[i + 1]
            pt2_y = frac_ys[i + 1]
            diff_x = pt2_x - pt1_x
            diff_y = pt2_y - pt1_y
            mid_x = pt1_x + diff_x / 2
            mid_y = pt1_y + diff_y / 2
            for pt_x, pt_y in zip([pt1_x, mid_x], [pt1_y, mid_y]):
                new_y = int((pt_y - min_y) / grid_const)
                new_x = (pt_x - min_x) / grid_const
                grid[(new_y, new_x)] = 1

    else:
        for pt_x, pt_y in zip(frac_xs, frac_ys):
            new_y = int((pt_y - min_y) / grid_const)
            new_x = int((pt_x - min_x) / grid_const)
            grid[(new_y, new_x)] = 1

    return (grid_const, grid)


def mark_inside_bfs(grid: np.ndarray):
    mid = grid.shape[0] // 2
    xq: deque[int] = deque()
    yq: deque[int] = deque()
    grid[(mid, mid)] = 2
    xq.append(mid)
    yq.append(mid)

    while xq:
        x = xq.popleft()
        y = yq.popleft()

        diffs = [(-1, 0), (1, 0), (0, -1), (0, 1)]
        for (dy, dx) in diffs:
            new_y, new_x = y + dy, x + dx
            match grid[(new_y, new_x)]:
                case 0:  # Outside
                    grid[(new_y, new_x)] = 2  # inside
                    xq.append(new_x)
                    yq.append(new_y)
                case _:  # All else cases
                    continue
    return grid
