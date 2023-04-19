# N(w) = number of eigenfrequencies below w (including degenerate)
# w = eigenfrequency
import json
import numpy as np
import matplotlib.pyplot as plt


def integrated_density_of_states(eigvals: list):
    return np.arange(len(eigvals)) + 1


def density_of_states(w: list, N: list):
    A = 1
    return A / (4 * np.pi) * w**2 - N


def main():
    with open("cache/eigvals_l4_1mids_100vals.json", "r") as f:
        eigvals = np.array(json.load(f)[::-1])
        print(len(eigvals))
    v = 1
    w = np.sqrt(eigvals) * v
    N = integrated_density_of_states(w)
    delta_N = density_of_states(w, N)
    plt.plot(w, delta_N)
    plt.xlabel(r"$\omega$")
    plt.ylabel(r"$\Delta N(\omega)$")
    plt.show()


if __name__ == "__main__":
    main()
