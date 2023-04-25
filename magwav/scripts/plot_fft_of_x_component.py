import matplotlib.pyplot as plt
import matplotlib as mpl
from pathlib import Path
import json
import numpy as np
from numpy.fft import fft2, rfft2, fftshift, fftfreq
from sketch_omega import plot_omega

cur_dir = Path(__file__).parent

fname = "x_components_a0.01"

with open(cur_dir / f"../plots/{fname}.json", "r") as readfile:
    components = json.load(readfile)

components = np.array(components)

ft = abs(rfft2(components))
shifted = fftshift(ft, axes=0)
cropped = shifted[
    shifted.shape[0] // 2 : shifted.shape[0] // 2 + shifted.shape[0] // 100, :
]

# filter = ft > ft.max() / 10
# larges = np.nonzero(filter)
# plt.scatter(larges[1], larges[0])

fig, ax = plt.subplots()
im = ax.imshow(cropped, aspect="auto", origin="lower", extent=(0, np.pi, 0, 10e16))
im.set_clim(None, 40_000)
ax.set_xlabel("k")
ax.set_ylabel("w")
# ax.set_ylim(bottom=0)
fig.colorbar(im)
hbar = 6.582e-16  # eV * s
w = lambda ka, J, dz: (2 * dz + 2 * J * (1 - np.cos(ka))) / hbar
ka = np.linspace(0, np.pi)
ax.plot(ka, w(ka, 10, 1) * 0.7, color="red", label="J=10, dz=1")
ax.legend()
plt.savefig(cur_dir / f"../plots/{fname}_wit_fit_J7_dz1.jpg")
plt.show()

# # Testing the plotting and analysis
# t = np.linspace(np.zeros(50), np.ones(50) * 100, 100, dtype=float)
# y = np.sin(t * 20)

# ft = np.abs(fft2(y))
# print(np.nonzero(ft))
# print(y)
# im = plt.imshow(ft, aspect="auto", interpolation="none")
# plt.colorbar(im)
# plt.show()
