import numpy as np
import matplotlib.pyplot as plt
import json
from scipy.interpolate import interp1d

# 读取数据

x_label = "Function Latency (ms)"
y_label = "CDF (%)"

colors = ['blue', 'green', 'red']
line_labels = ['Openfaas-empty-payload', 'Openfaas-512KB', 'Openfaas-4MB']
filenames = ["alu-flask-remote-0kb.trace",
             "alu-flask-remote-512kb.trace", "alu-flask-remote-4096kb.trace",]


def read_data(filename: str):
    with open(f"/home/yjn/rust_project/mslibos/baseline/alu-flask/{filename}", "r") as f:
        trace: dict = json.loads(f.readline())
        func: dict = json.loads(f.readline())
        dataset: dict = {}
        entry_num = len(func["comp_time"])

        funct_times = func["comp_time"]
        with_runtime = [end-start for start, end in list(
            zip(trace['watchdog_begin'], trace['watchdog-resp']))]
        openfaas_times = [total-runtime for runtime,
                          total in list(zip(with_runtime, trace['finish']))]

        # dataset['func'] = funct_times
        # dataset['func'] = sorted(funct_times)[entry_num//4:-entry_num//4]
        # dataset['openfaas'] = openfaas_times
        data = sorted(openfaas_times)
        return data


# 计算CDF
def CDF(data, label, color):
    sorted_data = np.sort(data)
    cumulative_prob = np.arange(
        1, len(sorted_data) + 1) / len(sorted_data) * 100

    # 使用样条插值
    f = interp1d(sorted_data, cumulative_prob, kind='linear')

    # 生成更平滑的数据点
    smooth_data = np.linspace(min(sorted_data), max(sorted_data), num=1000)
    smooth_cumulative_prob = f(smooth_data)

    # 绘制CDF图
    plt.plot(smooth_data, smooth_cumulative_prob, linestyle='-',
             linewidth=2, color=color, label=label)


if __name__ == '__main__':
    for i in range(3):
        data = read_data(filenames[i])
        CDF(data, line_labels[i], colors[i])

    with open(f"/home/yjn/rust_project/mslibos/baseline/alu-flask/alu-flask-remote-0kb.trace", "r") as f:
        data = json.loads(f.readlines()[1])["comp_time"]
        CDF(data, "func exec", 'black')

    # plt.xscale('log')  # 对 x 轴进行对数转换
    plt.xlabel(x_label, labelpad=4)
    plt.ylabel(y_label, labelpad=8)

    plt.grid(zorder=0, linestyle=":", axis="y")
    plt.legend()

    plt.show()
