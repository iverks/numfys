import json
import time
from pprint import pprint

import numpy as np
from fractal_generator import generate_fractal_drum, generate_from_line
from grid import grid_from_fractal, mark_inside_bfs
from plotty import plot_fractal, plot_grid, plot_sln, plot_sln_im
from solver import solve, solve_clamped, solve_higher_order


def main():
    side_length = 1.0
    depth = 3
    frac_xs, frac_ys = generate_fractal_drum(side_length, depth)
    print("Finished frac")
    # plot_fractal(frac_xs, frac_ys)
    # grid_const, grid = grid_from_fractal(frac_xs, frac_ys, side_length, depth, False)
    grid_const, grid = grid_from_fractal(frac_xs, frac_ys, side_length, depth, True)
    print("Finished grid")
    # plot_grid(grid)
    grid = mark_inside_bfs(grid)
    print("Finished marking")
    # plot_grid(grid)
    num_slns = 10
    start = time.time()
    # eigvals, eigfn = solve(grid, grid_const, num_slns, 1e-2)
    eigvals, eigfn = solve_higher_order(grid, grid_const, num_slns, 1e-2)
    # eigvals, eigfn = solve_clamped(grid, grid_const, num_slns, 1e-6)
    stop = time.time()
    print(f"Finished solving with in {stop - start} s")
    print("Eigvals:")
    for eig in eigvals:
        print(f"{eig:.2f} | {np.sqrt(eig):.2f}")

    sln_num = 3
    print(f"Plotting eigval = {eigvals[sln_num]}")
    plot_sln_im(eigfn[sln_num], grid)

    # for sln_num in range(5):
    #     print(f"Plotting eigval = {eigvals[sln_num]}")
    #     plot_sln(eigfn[sln_num], f"images/solved_l4_5pt_eig{eigvals[sln_num]:.2f}.png")


if __name__ == "__main__":
    main()
