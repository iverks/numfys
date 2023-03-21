"""
0 -> outside \n
1 -> wall \n
2 -> inside \n
"""

import numpy as np
from scipy.sparse import coo_matrix
from scipy.sparse.linalg import eigsh, eigs


def solve(
    grid: np.ndarray, grid_const: float, num_slns: int
) -> tuple[np.ndarray, np.ndarray]:
    n = grid.shape[0]  # Assume square matrix
    rows, cols, vals = [], [], []

    grid_const_squared = grid_const**2
    for i in range(n**2):
        y = i // n
        x = i % n

        if grid[y, x] != 2:
            continue

        rows.append(i)
        cols.append(y * n + x)
        vals.append(-4.0 / grid_const_squared)

        for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
            new_y = y + dy
            new_x = x + dx
            if not (0 <= new_x < n and 0 <= new_y < n):
                continue

            rows.append(i)
            cols.append(new_y * n + new_x)
            vals.append(1.0 / grid_const_squared)

    coom = coo_matrix((vals, (rows, cols)), shape=(n * n, n * n))
    csc = coom.tocsc()
    eigvals, eigfns = eigsh(csc, num_slns + 1)
    result_fns = []
    for num in range(num_slns):
        eigfn = eigfns[:, num_slns]
        result_fns.append(eigfn.reshape((n, n)))
    return (eigvals, result_fns)
