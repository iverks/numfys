import smumerix
import matplotlib.pyplot as plt
from matplotlib.collections import PatchCollection
from matplotlib.patches import Circle
import json
from pathlib import Path
from time import time
import numpy as np

curdir = Path(__file__).parent


def plot_snapshot(edg: smumerix.EventDrivenGas, init_speed):
    sizes = edg.get_sizes()
    xs, ys = edg.get_positions()
    circles = []
    for size, x, y in zip(sizes, xs, ys):
        circle = Circle((x, y), size)
        circles.append(circle)
    p = PatchCollection(circles)
    fig, ax = plt.subplots()
    ax.add_collection(p)
    fig.savefig(curdir / f"crater_w_speed_{init_speed}.png")


if False:
    data = []
    for idx, init_speed in enumerate(np.logspace(0.01, 10.0, 100)):
        edg = smumerix.EventDrivenGas.new_big_and_small(10000, init_speed, 0.0028, 0.5)
        initial_energy = edg.get_total_energy()
        light_es = []
        heavy_es = []

        edg.step_until_energy(initial_energy * 0.1)
        speeds = edg.get_speeds()
        num_collided = len([1 for speed in speeds if speed > 0])
        data.append((init_speed, num_collided))
        print(f"{idx} done")

    with open(curdir / "cache" / "task4.json", "w") as savefile:
        json.dump(data, savefile)

with open(curdir / "cache" / "task4.json", "r") as savefile:
    data = json.load(savefile)

init_speeds = []
num_collideds = []
for init_speed, num_collided in data:
    print(
        f"With {init_speed=}, {num_collided} of the particles collided before the energy was 0.1 times the initial energy"
    )
    init_speeds.append(init_speed)
    num_collideds.append(num_collided)

plt.plot(init_speeds, num_collideds)
plt.ylim(bottom=0)
plt.xscale("log")
plt.title("State after 0.1 of initial kinetic energy is left")
plt.xlabel("Initial speed of particle")
plt.ylabel("No. of particles with nonzero speed")
plt.savefig(curdir / "collisions_per_init_speed.png")

plt.show()
