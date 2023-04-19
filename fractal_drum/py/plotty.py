import matplotlib.pyplot as plt
from matplotlib.colors import ColorConverter
import numpy as np
import matplotlib as mpl


def plot_fractal(xs: list[float], ys: list[float]):
    plt.plot(xs, ys)
    plt.show()


def plot_grid(grid: np.ndarray):
    plt.imshow(grid, interpolation="none", cmap="binary_r")
    # plt.savefig("images/mmm")
    plt.show()


def plot_sln(grid: np.ndarray, fname: str = None):
    fig, ax = plt.subplots(subplot_kw={"projection": "3d"})
    x = np.arange(grid.shape[0])
    y = np.arange(grid.shape[1])
    x, y = np.meshgrid(x, y)
    ax.plot_surface(x, y, grid, cmap="magma")

    if fname:
        plt.savefig(fname)
    else:
        plt.show()


def plot_sln_im(grid: np.ndarray, fractal: np.ndarray | None):
    plt.imshow(grid)
    plt.contour(grid)
    if fractal is not None:
        transparent = ColorConverter.to_rgba("black", 0)
        white = ColorConverter.to_rgba("white")
        cmap = mpl.colors.LinearSegmentedColormap.from_list(
            "mycmap", [transparent, white], 256
        )

        fractal[fractal == 2] = 0
        plt.imshow(fractal, cmap=cmap, interpolation="none")
    plt.show()
