import matplotlib.pyplot as plt
from pathlib import Path
import json
import numpy as np

cur_dir = Path(__file__).parent


def read_file(fname: str):
    with open(cur_dir / f"../plots/{fname}.json", "r") as readfile:
        avgs = json.load(readfile)

    return np.array(avgs)


def get_Mavg(arr: np.ndarray):
    derivative = np.gradient(arr)
    border = -1
    for i, elem in enumerate(derivative):
        if np.abs(elem) < 1e-7:
            border = i
            break
    mavg = np.average(arr[border:])
    return border, mavg


def task231():
    save = True
    for fname in ["avgs_2_3_1", "avgs_2_3_2"]:
        t = 0.0 if fname == "avgs_2_3_1" else 0.1
        avgs = read_file(fname)

        border, mavg = get_Mavg(avgs)

        time = np.arange(0, len(avgs), 1.0)

        fig, ax = plt.subplots()
        ax.annotate(
            f"$J = 10.0 meV$\n$k_B T = {t} J$",
            (0.1, 1 - 0.1),
            color="orange",
            xycoords="axes fraction",
        )
        ax.plot(time, avgs, color="green", label=r"$M(T, t)$")
        ax.hlines(
            mavg,
            time[border],
            time[-1],
            colors="purple",
            linestyles="dashed",
            label=r"$M_{avg}$",
        )

        ax.set_xlabel("time [ns]")
        ax.set_ylabel("M(T, t)")
        ax.legend()
        if save:
            plt.savefig(cur_dir / f"../images/2.3/{fname}.jpg", dpi=200)
            plt.close()
        else:
            plt.show()


def task233():
    save = True
    fname = "avgs_2_3_3_10k"
    with open(cur_dir / f"../plots/{fname}.json", "r") as readfile:
        all_avgs: dict[str, list[float]] = json.load(readfile)
    Ts = []
    avgs = []
    fig, ax = plt.subplots()
    for T in sorted(all_avgs, key=lambda el: float(el)):
        xs = all_avgs[T]
        Ts.append(float(T))
        border, mavg = get_Mavg(xs)
        avgs.append(mavg)
        time = np.arange(0, len(xs), 1.0)

        color = next(ax._get_lines.prop_cycler)["color"]
        ax.plot(xs, label=r"$k_BT =$" + T, color=color)
        ax.hlines(mavg, border, len(xs), colors=color, linestyles="dashed")
    ax.set_title("J=10 meV, dz=10 meV")
    ax.set_xlabel("num timesteps")
    ax.set_ylabel("M")
    # ax.legend()
    if save:
        plt.savefig(
            cur_dir / f"../images/2.3/m_per_time.jpg",
            dpi=200,
            pad_inches=0,
            bbox_inches="tight",
        )
        plt.close()
    else:
        plt.show()

    plt.plot(Ts, avgs)
    plt.vlines(17, 0.0, 1.0, colors="green")
    plt.title("J=10 meV, dz=10 meV")
    plt.xlabel("T [meV]")
    plt.ylabel("M average")
    if save:
        plt.savefig(
            cur_dir / f"../images/2.3/mavg_per_T.jpg",
            dpi=200,
            pad_inches=0,
            bbox_inches="tight",
        )
        plt.close()
    else:
        plt.show()


# task231()
task233()
