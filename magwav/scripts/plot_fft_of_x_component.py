import matplotlib.pyplot as plt
from pathlib import Path
import json
import numpy as np
from numpy.fft import fft2

cur_dir = Path(__file__).parent

with open(cur_dir / "../plots/x_components.json", "r") as readfile:
    components = json.load(readfile)

components = np.array(components).T

ft = abs(fft2(components))

filter = ft > ft.max() / 100

im = plt.imshow(ft, aspect="auto")
# im = plt.imshow(filter, aspect="auto")
plt.xlabel("k")
plt.ylabel("w")
plt.colorbar(im)

plt.show()
