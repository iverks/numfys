import matplotlib.pyplot as plt
import numpy as np


def plot_fractal(xs: list[float], ys: list[float]):
    plt.plot(xs, ys)
    plt.show()


def plot_grid(grid: np.ndarray):
    plt.imshow(grid)
    # plt.savefig("images/mmm")
    plt.show()


def plot_sln(grid: np.ndarray, fname: str = ""):
    fig, ax = plt.subplots(subplot_kw={"projection": "3d"})
    x = np.arange(grid.shape[0])
    y = np.arange(grid.shape[1])
    x, y = np.meshgrid(x, y)
    ax.plot_surface(x, y, grid, cmap="magma")

    plt.show()
    # plt.savefig(fname)
