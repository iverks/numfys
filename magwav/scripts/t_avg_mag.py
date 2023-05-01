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
    fname = "avgs_2_3_1_tmp"
    avgs0 = read_file(fname)
    avgs1 = read_file("avgs_2_3_2")

    border0, mavg0 = get_Mavg(avgs0)
    border1, mavg1 = get_Mavg(avgs1)

    time = np.arange(0, len(avgs0), 1.0)

    fig, ax = plt.subplots()
    ax.plot(time, avgs0, color="orange", label=r"$k_BT=0.0$")
    ax.hlines(
        mavg0,
        time[border0],
        time[-1],
        colors="orange",
        linestyles="dashed",
        label=r"Mavg $k_BT=0.0$",
    )

    ax.plot(time, avgs1, color="blue", label=r"$k_BT=0.1$")
    ax.hlines(
        mavg1,
        time[border1],
        time[-1],
        colors="blue",
        linestyles="dashed",
        label=r"Mavg $k_BT=0.1$",
    )

    ax.set_xlabel("time [ns]")
    ax.set_ylabel("M(T, t)")
    ax.legend()
    # plt.savefig(cur_dir / f"../images/2.3/{fname}.jpg", dpi=200)
    plt.show()


def task233():
    fname = "avgs_2_3_3_tmp"
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
        ax.plot(time, xs, label=r"$k_BT =$" + T, color=color)
        ax.hlines(mavg, border, len(xs), colors=color, linestyles="dashed")
    ax.set_title("J=10 meV, dz=10 meV")
    ax.set_xlabel("M")
    ax.set_ylabel("t [ns]")
    # ax.legend()
    # plt.savefig(cur_dir / f"../images/2.3/m_per_time.jpg", dpi=200)
    plt.show()
    # line = all_avgs["0.00"]
    # border, mavg = get_Mavg(line)

    # print(Ts)
    # plt.bar(Ts, avgs)
    plt.plot(Ts, avgs)
    plt.vlines(17, 0.0, 1.0, colors="green")
    plt.title("J=10 meV, dz=10 meV")
    plt.xlabel("T [meV]")
    plt.ylabel("M average")
    # plt.savefig(cur_dir / f"../images/2.3/mavg_per_T.jpg", dpi=200)
    plt.show()


# task231()
task233()
