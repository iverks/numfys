from fractal_generator import generate_fractal_drum, generate_from_line
from grid import grid_from_fractal, mark_inside_bfs
from plotty import plot_fractal, plot_grid, plot_sln
from solver import solve
import json
import time


def main():
    side_length = 1.0
    depth = 5
    start = time.time()
    frac_xs, frac_ys = generate_fractal_drum(side_length, depth)
    print("Finished frac")
    # plot_fractal(frac_xs, frac_ys)
    grid_const, grid = grid_from_fractal(frac_xs, frac_ys, side_length, depth, False)
    print("Finished grid")
    # plot_grid(grid)
    grid = mark_inside_bfs(grid)
    print("Finished marking")
    stop = time.time()
    print(f"Took {stop - start}")
    plot_grid(grid)
    # num_slns = 10
    # start = time.time()
    # eigvals, eigfn = solve(grid, grid_const, num_slns)
    # stop = time.time()
    # print(f"Finished solving with eigvals {eigvals} in {stop - start}")
    # sln_num = -5
    # print(f"Plotting eigval = {eigvals[sln_num]}")
    # plot_sln(eigfn[sln_num])


if __name__ == "__main__":
    main()
