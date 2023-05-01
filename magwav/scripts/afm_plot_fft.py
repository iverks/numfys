import matplotlib.pyplot as plt
import matplotlib as mpl
from pathlib import Path
import json
import numpy as np
from numpy.fft import fft2, fftshift

cur_dir = Path(__file__).parent

fname = "x_afm_J-30_dz0.01_a0.001"

with open(cur_dir / f"../plots/{fname}.json", "r") as readfile:
    components = json.load(readfile)

components = np.array(components)

ft = abs(fft2(components))
shifted = fftshift(ft, axes=1)
rng = shifted.shape[0] // 100
fromx = shifted.shape[1] // 4 + 1
tox = shifted.shape[1] * 3 // 4 + 1
cropped = shifted[:rng, fromx:tox]

fig, ax = plt.subplots()
im = ax.imshow(
    cropped,
    aspect="auto",
    origin="lower",
    extent=(-np.pi / 2, np.pi / 2, 0, 1e17),
    interpolation="none",
)
im.set_clim(0, 2_000)
ax.set_xlabel("k")
ax.set_ylabel("w")
fig.colorbar(im)
ax.set_ylim(bottom=0)
# plt.savefig(cur_dir / f"../images/2.2.6/{fname}.jpg", dpi=200)
plt.show()
