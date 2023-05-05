import matplotlib.pyplot as plt
import numpy as np


def plot_omega():
    hbar = 6.582e-16  # eV * s
    w = lambda ka, J, dz: (2 * dz + 2 * J * (1 - np.cos(ka))) / hbar
    ka = np.linspace(0, np.pi)
    plt.plot(ka, w(ka, 10, 3), label="J=10")
    plt.plot(ka, w(ka, 20, 3), label="J=20")
    plt.ylim(bottom=0)
    plt.xlabel("ka")
    plt.ylabel(r"$\omega$")
    plt.legend()
    # plt.savefig(
    #     "images/omega_sketch.jpg",
    #     dpi=200,
    #     pad_inches=0,
    #     bbox_inches="tight",
    # )


def main():
    plot_omega()
    plt.show()


if __name__ == "__main__":
    main()
