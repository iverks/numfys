import matplotlib.pyplot as plt
from pathlib import Path
import json
import numpy as np
from numpy.fft import fft2, fftshift

cur_dir = Path(__file__).parent

save = True
j = 10
for b in [1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0]:
    fname = f"x_w_mag_j{j}_b{b}"

    with open(cur_dir / f"../plots/{fname}.json", "r") as readfile:
        components = json.load(readfile)

    components = np.array(components)

    ft = abs(fft2(components))
    shifted = fftshift(ft, axes=1)
    rng = shifted.shape[0] // 500
    cropped = shifted[:rng, :]

    fig, ax = plt.subplots()
    im = ax.imshow(
        cropped, aspect="auto", origin="lower", extent=(-np.pi, np.pi, 0, 1e17)
    )
    im.set_clim(None, 35000)
    ax.set_xlabel("ka")
    ax.set_ylabel("$\omega$")
    fig.colorbar(im)
    ax.set_ylim(bottom=0)
    ax.annotate(
        f"J = {j} meV\nB = {b} meV",
        (0.1, 1 - 0.1),
        color="orange",
        xycoords="axes fraction",
    )
    if save:
        plt.savefig(
            cur_dir / f"../images/2.2.5/{fname}.jpg",
            dpi=200,
            pad_inches=0,
            bbox_inches="tight",
        )
        plt.close()
    else:
        plt.show()
