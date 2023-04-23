# N(w) = number of eigenfrequencies below w (including degenerate)
# w = eigenfrequency
import json
import numpy as np
import matplotlib.pyplot as plt
from scipy.optimize import curve_fit


def integrated_density_of_states(eigvals: np.array):
    return np.arange(len(eigvals)) + 1


def density_of_states(w: np.array, N: np.array):
    A = 1
    return A / (4 * np.pi) * w**2 - N


def main():
    # filename = "cache/eigvals_l2_2mids_1000vals.json"
    filename = "cache/eigvals_l2_3mids_1000vals.json"
    # filename = "cache/eigvals_l3_0mids_1000vals.json"
    # filename = "cache/eigvals_l3_1mids_1000vals.json"
    # filename = "cache/eigvals_l3_2mids_200vals.json"  # 2
    # filename = "cache/eigvals_l3_3mids_200vals.json"
    # filename = "cache/eigvals_l3_4mids_200vals.json"
    # filename = "cache/eigvals_l4_1mids_600vals.json"
    with open(filename, "r") as f:
        eigvals = np.array(json.load(f)[::-1])
        # print(len(eigvals))
    # eigvals = eigvals[:120]
    v = 1
    w = np.sqrt(eigvals) * v
    N = integrated_density_of_states(w)
    delta_N = density_of_states(w, N)

    (d, k), _ = curve_fit(lambda w, d, k: k * w**d, w[:120], delta_N[:120], [2, 0])
    print(d)

    plt.plot(w, delta_N, label="experimental values")
    plt.plot(w, k * w**d, label=rf"$\Delta N = {k:.2f} \cdot \omega ^{{{d:.2f}}}$")
    plt.xlabel(r"$\omega$")
    plt.ylabel(r"$\Delta N(\omega)$")
    plt.xlim(left=0)
    plt.ylim(bottom=0)
    plt.legend()
    plt.savefig(f"images/finding_d.png")
    plt.show()


if __name__ == "__main__":
    main()
