import numpy as np
import matplotlib.pyplot as plt
import json
from scipy.interpolate import interp1d
import re

# fig = plt.figure(figsize=(7, 4), dpi=300)
plt.rcParams.update({'font.size': 22})
# plt.subplots_adjust(bottom=.20, left=.20)

# 读取数据
x_label = "end-to-end Latency (ms)"
y_label = "CDF (%)"

#          blue        red         green
colors = ['#2C73D2', '#CA4829', '#008D4A']
linestype = ["-", "--"]
line_labels = ['AlloyStack-C1', 'AlloyStack-C3',
               'AlloyStack-C5', 'OpenFaaS-C1', 'OpenFaaS-C3', 'OpenFaaS-C5']


def read_data(app_name: str, filename: str,):
    with open(f"/home/yjn/rust_project/mslibos/baseline/end2end/{app_name}_cdf_logs/{filename}", "r") as f:
        trace = f.readlines()
        end2end = []

        pattern = r": (\d+)(?:\.\d+)?"
        for line in trace:
            match = re.search(pattern, line)
            if match:
                end2end.append(int(match.group(1)))

        # dataset['func'] = funct_times
        # dataset['func'] = sorted(funct_times)[entry_num//4:-entry_num//4]
        # dataset['openfaas'] = openfaas_times
        data = sorted(end2end)
        return data


lines = []

# 计算CDF


def CDF(axs, data, label, color, linestyle,):
    sorted_data = np.sort(data)
    cumulative_prob = np.arange(
        1, len(sorted_data) + 1) / len(sorted_data) * 100
    # 绘制CDF图
    line, = axs.plot(data, cumulative_prob, linestyle=linestyle,
                     linewidth=3, color=color, label=label)
    lines.append(line)

    axs.grid(ls="--", zorder=1)


def main():
    global line_labels
    fig, axs = plt.subplots(2, 3, figsize=(17, 8))  # 1行3列
    axs = axs.flatten()

    for idx, datasize in enumerate(['10', '100', '300']):
        filenames = [f"as_c1_{datasize}mb.txt", f"as_c3_{datasize}mb.txt", f"as_c5_{datasize}mb.txt",
                     f"of_c1_{datasize}mb.txt", f"of_c3_{datasize}mb.txt", f"of_c5_{datasize}mb.txt",]

        for i in range(6):
            data = read_data("wc", filenames[i])
            print(len(data), data)
            CDF(axs[idx], data, line_labels[i], colors[i % 3],
                linestype[0 if i < 3 else 1])
            title = f"{datasize}MB, Latency (ms)"
            print(f"WC: {title}: {sum(data)/len(data)}")
            axs[idx].set_xlabel(title, labelpad=15)

    # plt.xscale('log')  # 对 x 轴进行对数转换
    # plt.legend()
    # fig.xlabel(x_label, labelpad=4)
    # fig.ylabel(y_label, labelpad=8)
    axs[0].set_ylabel("WC CDF (%)")

    for idx, datasize in enumerate(['1', "25", "50"]):
        idx += 3
        filenames = [f"as_c1_{datasize}mb.txt", f"as_c3_{datasize}mb.txt", f"as_c5_{datasize}mb.txt",
                     f"of_c1_{datasize}mb.txt", f"of_c3_{datasize}mb.txt", f"of_c5_{datasize}mb.txt",]
        
        for i in range(len(filenames)):
            data = read_data("ps", filenames[i])
            # print(len(data), data)
            CDF(axs[idx], data, line_labels[i], colors[i % 3],
                linestype[0 if i < 3 else 1])
            title = f"{datasize}MB, Latency (ms)"
            print(f"{filenames[i]}: {sum(data)/len(data)}")
            axs[idx].set_xlabel(f"title", labelpad=15)
            # axs[idx].set_xscale("log")

    axs[3].set_ylabel("PS CDF (%)")
    # fig.subplots_adjust(top=0.85, left=0.05, right=0.95, wspace=0.25, hspace=0.35)
    fig.legend(lines, line_labels, frameon=False, loc="upper center", ncol=3)
    plt.tight_layout(rect=[0, 0, 1, 0.85])

    plt.savefig(f"/home/yjn/Downloads/sec6_end2end_mr.pdf")
    # plt.show()


if __name__ == '__main__':
    main()
