import matplotlib.pyplot as plt
import matplotlib as mpl
from pathlib import Path
import json
import numpy as np
from numpy.fft import fft2, fftshift

cur_dir = Path(__file__).parent

save = True
J = -30
for b in [0, 0.05]:
    fname = f"x_afm_j{J}_b{b}j"

    with open(cur_dir / f"../plots/{fname}.json", "r") as readfile:
        components = json.load(readfile)

    components = np.array(components)

    ft = abs(fft2(components))
    shifted = fftshift(ft, axes=1)
    rng = shifted.shape[0] // 500
    fromy = 0
    extentmin = 0
    extentmax = 1e17

    if b == 0.05:
        # Scale the matrix slicing and the plot labels exacly the same amount
        fromy = rng * 30
        rng *= 10
        extentmin += extentmax * 30
        extentmax = extentmin + extentmax * 10

    fromx = shifted.shape[1] // 4 + 1
    tox = shifted.shape[1] * 3 // 4 + 1
    cropped = shifted[fromy : fromy + rng, fromx:tox]

    fig, ax = plt.subplots()
    im = ax.imshow(
        cropped,
        aspect="auto",
        origin="lower",
        extent=(-np.pi / 2, np.pi / 2, extentmin, extentmax),
        interpolation="none",
    )
    im.set_clim(0, 2_000)
    ax.set_xlabel("ka")
    ax.set_ylabel("$\omega$")
    fig.colorbar(im)
    ax.set_ylim(bottom=extentmin)
    ax.annotate(
        f"J = {J} meV\nB = {b} J",
        (0.1, 1 - 0.1),
        color="orange",
        xycoords="axes fraction",
    )
    if save:
        plt.savefig(
            cur_dir / f"../images/2.2.6/{fname}.jpg",
            dpi=200,
            pad_inches=0,
            bbox_inches="tight",
        )
        plt.close()
    else:
        plt.show()
