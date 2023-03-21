import matplotlib.pyplot as plt
from pathlib import Path
import json
import numpy as np

cur_dir = Path(__file__).parent

with open(cur_dir / "../plots/x_components.json", "r") as readfile:
    components = json.load(readfile)

X = np.arange(len(components[0]))
T = np.arange(len(components))
X, T = np.meshgrid(X, T)
Z = np.array(components)


ax = plt.figure().add_subplot(projection="3d")
ax.plot_surface(X, T, Z)
plt.show()
