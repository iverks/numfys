from fractal_generator import generate_fractal_drum, generate_from_line
from grid import grid_from_fractal, mark_inside_bfs
from plotty import plot_fractal, plot_grid, plot_sln
from solver import solve, solve_higher_order
import json
import time


def main():
    side_length = 1.0
    depth = 3
    frac_xs, frac_ys = generate_fractal_drum(side_length, depth)
    print("Finished frac")
    # plot_fractal(frac_xs, frac_ys)
    grid_const, grid = grid_from_fractal(frac_xs, frac_ys, side_length, depth, False)
    print("Finished grid")
    # plot_grid(grid)
    grid = mark_inside_bfs(grid)
    print("Finished marking")
    # plot_grid(grid)
    num_slns = 13
    start = time.time()
    # eigvals, eigfn = solve_higher_order(grid, grid_const, num_slns)
    eigvals, eigfn = solve(grid, grid_const, num_slns)
    stop = time.time()
    print(f"Finished solving with eigvals {eigvals} in {stop - start} s")

    sln_num = 12
    print(f"Plotting eigval = {eigvals[sln_num]}")
    plot_sln(eigfn[sln_num])

    # for sln_num in range(10):
    #     print(f"Plotting eigval = {eigvals[sln_num]}")
    #     plot_sln(eigfn[sln_num])


if __name__ == "__main__":
    main()
