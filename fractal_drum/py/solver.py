"""
0 -> outside \n
1 -> wall \n
2 -> inside \n
https://web.media.mit.edu/~crtaylor/calculator.html
"""

import numpy as np
from scipy.sparse import coo_matrix, csc_matrix
from scipy.sparse.linalg import eigs, eigsh


def remove_zeros(matty: csc_matrix) -> tuple[csc_matrix, np.ndarray]:
    """
    Removes rows and columns that only have zeros from matrix. The matrix shape ends up nonsensical and needs to be intelligently reverted after being solved using the indices returned from this function.

        Args:
            matty (csc_matrix): sparse matrix containing equation to be solved

        Returns:
            csc_matrix: resulting matrix with no zero rows
            ndarray: boolean array of indices in 1d where we have nonzero values
    """
    indices = matty.getnnz(0) > 0
    matty = matty[indices][:, indices]
    return matty, indices


def add_back_zeros(eigfn: np.ndarray, indices: np.ndarray, n: int) -> np.ndarray:
    """
    Insert zeros back into array in order to restore logical shape.

        Args:
            eigfn (ndarray): A single eigenvector from eigsh, before reshaping
            indices (ndarray): The indices returned from remove_zeros
            n (int): Size of a single axis on the original grid

        Returns:
            ndarray: Eigenvector with zeros at correct places
    """
    res = np.zeros(n**2)
    res[indices] = eigfn
    return res


def solve(
    grid: np.ndarray, grid_const: float, num_slns: int, tol: float, get_fns: bool = True
) -> tuple[np.ndarray, np.ndarray]:
    n = grid.shape[0]  # Assume square matrix
    rows, cols, vals = [], [], []

    common_const = 1 / (grid_const**2)
    for i in range(n**2):
        y = i // n
        x = i % n

        if grid[y, x] != 2:
            continue

        rows.append(i)
        cols.append(y * n + x)
        vals.append(4 * common_const)

        for dy, dx in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
            new_y = y + dy
            new_x = x + dx
            if not (0 <= new_x < n and 0 <= new_y < n):
                continue
            # if grid[new_y, new_x] == 0:
            #     continue
            if grid[new_y, new_x] != 2:
                continue

            rows.append(i)
            cols.append(new_y * n + new_x)
            vals.append(-1.0 * common_const)

    # coom: coo_matrix = coo_matrix((vals, (rows, cols)), shape=(n * n, n * n))
    # csc: csc_matrix = coom.tocsc()
    csc = csc_matrix((vals, (rows, cols)), shape=(n * n, n * n))

    # Remove 0 rows takk til viljar
    csc, indices = remove_zeros(csc)
    print(csc.shape)
    print(csc.data.nbytes)

    if not get_fns:
        return (
            eigsh(csc, num_slns, which="SA", tol=tol, return_eigenvectors=get_fns),
            [],
        )

    eigvals, eigfns = eigsh(csc, num_slns, which="SA", tol=tol)

    print(eigfns.nbytes)

    result_fns = []
    for num in range(num_slns):
        eigfn = eigfns[:, num]
        # Add back 0 rows takk til viljar
        eigfn = add_back_zeros(eigfn, indices, n)

        result_fns.append(eigfn.reshape((n, n)))
    return (eigvals, result_fns)


"""
0  0  0 -2 0 0
0  0  0 27 0 0
-2 27 -270 490  

"""


def solve_higher_order(
    grid: np.ndarray, grid_const: float, num_slns: int, tol: float
) -> tuple[np.ndarray, np.ndarray]:
    n = grid.shape[0]  # Assume square matrix
    rows, cols, vals = [], [], []

    common_const = 1 / (180 * grid_const**2)
    for i in range(n**2):
        y = i // n
        x = i % n

        if grid[y, x] != 2:
            continue

        rows.append(i)
        cols.append(y * n + x)
        vals.append((490 * 2) * common_const)

        for dy, dx in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
            for dm, coeff in zip(range(1, 3 + 1), [-270, 27, -2]):
                new_y = y + dy * dm
                new_x = x + dx * dm
                if not (0 <= new_x < n and 0 <= new_y < n):
                    continue
                if grid[new_y, new_x] != 2:
                    continue

                rows.append(i)
                cols.append(new_y * n + new_x)
                vals.append(coeff * common_const)

    coom: coo_matrix = coo_matrix((vals, (rows, cols)), shape=(n * n, n * n))
    csc: csc_matrix = coom.tocsc()

    # Remove 0 rows takk til viljar
    csc, indices = remove_zeros(csc)

    eigvals, eigfns = eigsh(csc, num_slns, which="SM", tol=tol)

    result_fns = []
    for num in range(num_slns):
        eigfn = eigfns[:, num]
        eigfn = add_back_zeros(eigfn, indices, n)
        result_fns.append(eigfn.reshape((n, n)))
    return (eigvals, result_fns)


def solve_clamped(grid: np.ndarray, grid_const: float, num_slns: int, tol: float):
    """stencil yoinked from http://rodolphe-vaillant.fr/entry/57/2d-biharmonic-stencil-aka-bilaplacian-operator
    0  0  1  0  0
    0  2 -8  2  0
    1 -8 20 -8  1
    0  2 -8  2  0
    0  0  1  0  0
    """
    n = grid.shape[0]  # Assume square matrix
    rows, cols, vals = [], [], []

    common_const = 1 / (grid_const**4)
    for i in range(n**2):
        y = i // n
        x = i % n

        if grid[y, x] != 2:
            continue

        rows.append(i)
        cols.append(y * n + x)
        vals.append((56 * 2) * common_const)

        coeffs = np.array(
            [
                [0, 0, 1, 0, 0],
                [0, 2, -8, 2, 0],
                [1, -8, 20, -8, 1],
                [0, 2, -8, 2, 0],
                [0, 0, 1, 0, 0],
            ]
        )
        for dy in range(len(coeffs)):
            for dx in range(len(coeffs[dy])):
                new_y = y + dy - 2
                new_x = x + dx - 2
                if not (0 <= new_x < n and 0 <= new_y < n):
                    continue
                if grid[new_y, new_x] != 2:
                    continue
                coeff = coeffs[dy, dx]
                if coeff == 0:
                    continue

                rows.append(i)
                cols.append(new_y * n + new_x)
                vals.append(coeff * common_const)

    coom: coo_matrix = coo_matrix((vals, (rows, cols)), shape=(n * n, n * n))
    csc: csc_matrix = coom.tocsc()

    csc, indices = remove_zeros(csc)

    eigvals, eigfns = eigsh(csc, num_slns, which="SA", tol=tol)

    result_fns = []
    for num in range(num_slns):
        eigfn = eigfns[:, num]
        eigfn = add_back_zeros(eigfn, indices, n)
        result_fns.append(eigfn.reshape((n, n)))
    return (eigvals, result_fns)
