import matplotlib.pyplot as plt
import matplotlib as mpl
from pathlib import Path
import json
import numpy as np
from numpy.fft import fft2, fftshift

cur_dir = Path(__file__).parent

fname = "x_w_mag_B0.11_J10_dz3_a0.001"
fname = "x_w_mag_B0.2_J10_dz3_a0.001"

with open(cur_dir / f"../plots/{fname}.json", "r") as readfile:
    components = json.load(readfile)

components = np.array(components)

ft = abs(fft2(components))
shifted = fftshift(ft, axes=1)
rng = shifted.shape[0] // 100 * 4
cropped = shifted[:rng, :]

fig, ax = plt.subplots()
im = ax.imshow(cropped, aspect="auto", origin="lower", extent=(-np.pi, np.pi, 0, 4e17))
# im.set_clim(None, 40_000)
ax.set_xlabel("k")
ax.set_ylabel("w")
# ax.set_ylim(bottom=0)
fig.colorbar(im)
# hbar = 6.582e-16  # eV * s
# w = lambda ka, J, dz: (2 * dz + 2 * J * (1 - np.cos(ka))) / hbar
# ka = np.linspace(-np.pi, np.pi)
# J = -10
# dz = 25
# ax.plot(ka, w(ka, J, dz) * 0.7 + 0.55e17, color="red", label=f"J={J}, dz={dz}")
# ax.legend()
ax.set_ylim(bottom=0)
# plt.savefig(cur_dir / f"../images/2.2.5/{fname}.jpg")
plt.show()
