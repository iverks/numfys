import matplotlib.pyplot as plt
from pathlib import Path
import json
import numpy as np

cur_dir = Path(__file__).parent

with open(cur_dir / "../plots/x_components.json", "r") as readfile:
    components = json.load(readfile)

Z = np.array(components).T
# Z = Z[:, ::400]
X = np.arange(Z.shape[1])
T = np.arange(Z.shape[0])
X, T = np.meshgrid(X, T)

# ax = plt.figure().add_subplot(projection="3d")
# ax.plot_surface(X, T, Z)

im = plt.imshow(Z, aspect="auto")
plt.xlabel("time")
plt.ylabel("y-axis")
plt.colorbar(im)

# plt.plot(Z[25])

plt.show()
