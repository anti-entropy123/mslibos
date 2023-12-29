#!python3

import json

import matplotlib.pyplot as plt

import numpy as np
import pandas as pd
import seaborn as sns

import matplotlib.font_manager
matplotlib.rcParams["font.family"] = 'Helvetica'
matplotlib.rcParams['pdf.fonttype'] = 42
matplotlib.rcParams['ps.fonttype'] = 42

# 读取数据
with open("/home/yjn/rust_project/mslibos/baseline/alu-flask/alu-flask-remote.trace", "r") as f:
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
    dataset['openfaas'] = sorted(openfaas_times)

# with open("App4-java-lats.csv", "r") as f:
#     lines2 = f.readlines()

data = []


for label, lantencies in dataset.items():
    for lat in lantencies:
        data.append([label, "Remote", lat])


# for i, line in enumerate(lines2):
#     splits = line.split(":")

#     num_process = "%sCPU" % int(splits[0])

#     lats = splits[1].split(",")

#     for lat in lats:
#         data.append([num_process, "Java Thread", int(lat)])


# 根据多维数组创建pandas的DataFrame
cols = ["label", "deploy", "latency"]

df = pd.DataFrame(data=data, columns=cols)


fig = plt.figure(figsize=(2.325, 1.8), dpi=300)


plt.subplots_adjust(hspace=0.05, wspace=None, top=0.975,
                    bottom=0.13, left=0.18, right=0.995)

text_fontdict = {"size": 12}

# 根据属性设定颜色，即上面的n

my_pal = {
    "func": "#08519C",
    "openfaas": "#6BAED6",
    # "netes": "#3182BD",
    # "4CPU": "#C6DBEF",
}

plt.grid(axis="y", linestyle="--", zorder=1)


# 使用sns库绘制箱型图，以lang作为x轴的大分组，latency为y轴的值，n为每个x轴组内的小组划分属性
sns.boxplot(x="deploy", y="latency", hue="label", data=df,
            linewidth=0.5,
            whis=100,
            palette=my_pal,
            # showfliers=False
            )

# plt.xlim(0.5, 6.5)

# plt.yticks([])
plt.gca().yaxis.set_tick_params(length=1.5, pad=0)
plt.ylabel("Latency (ms)", fontdict=text_fontdict, labelpad=0)

axBox, axLabel = plt.gca().get_legend_handles_labels()

plt.legend(axBox, axLabel, ncol=1,
           # frameon=False,
           # loc=(0.15, 0.85),
           columnspacing=1
           )


# plt.savefig('boxplot.pdf')
plt.show()
