import matplotlib.pyplot as plt
import matplotlib as mpl
from pathlib import Path
import json
import numpy as np
from numpy.fft import fft2, fftshift

cur_dir = Path(__file__).parent

# fname = "x_w_mag_B0.1_J10_dz3_a0.001"
# fname = "x_w_mag_B0.11_J10_dz3_a0.001"
fname = "x_w_mag_B0.14_J10_dz3_a0.001"
# fname = "x_w_mag_B0.2_J10_dz3_a0.001"

with open(cur_dir / f"../plots/{fname}.json", "r") as readfile:
    components = json.load(readfile)

components = np.array(components)

ft = abs(fft2(components))
shifted = fftshift(ft, axes=1)
rng = shifted.shape[0] // 100 * 4
cropped = shifted[:rng, :]

fig, ax = plt.subplots()
im = ax.imshow(cropped, aspect="auto", origin="lower", extent=(-np.pi, np.pi, 0, 4e17))
ax.set_xlabel("k")
ax.set_ylabel("w")
fig.colorbar(im)
ax.set_ylim(bottom=0)
# plt.savefig(cur_dir / f"../images/2.2.5/{fname}.jpg", dpi=200)
plt.show()
