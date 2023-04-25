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

im = plt.imshow(cropped, aspect="auto", extent=(0, np.pi, 0, 10e-17), origin="lower")
plt.xlabel("k")
plt.ylabel("w")
plt.ylim(bottom=0)
plt.clim(None, 40_000)
plt.colorbar(im)
hbar = 6.582e-16  # eV * s
w = lambda ka, J, dz: (2 * dz + 2 * J * (1 - np.cos(ka))) / hbar
ka = np.linspace(0, np.pi)
plt.plot(ka, w(ka, 10, 3), color="red")
# plt.savefig(cur_dir / f"../plots/{fname}.jpg")
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
