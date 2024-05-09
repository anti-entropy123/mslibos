import matplotlib.pyplot as plt
import matplotlib as mpl
import numpy as np
import math
import matplotlib.font_manager

matplotlib.rcParams["font.family"] = 'Helvetica'
matplotlib.rcParams['pdf.fonttype'] = 42
matplotlib.rcParams['ps.fonttype'] = 42
plt.rcParams.update({'font.size': 16})
# 线宽
linewidth = 0.8
mpl.rcParams['hatch.linewidth'] = linewidth
fig = plt.figure(figsize=(7, 4), dpi=300)
plt.subplots_adjust(hspace=0.05, wspace=None, top=0.82, bottom=0.15, left=0.13, right=0.85)

# 数据
categories  = ['4KB', '64KB', '1MB', '16MB', '256MB']
alloy_stack = [411  , 407   , 540  , 1993  , 27317  ]
as_preload  = [40.7 , 47.4  , 173.8, 1740  , 26400  ]
faasm       = [7100 , 7820  , 8180 , 29000 , 351660 ]

labels = ['AS', 'AS-preload', "Faasm"]

# 绘制柱状图
bar_width = 0.25
index = range(len(categories))

back_colors = ["#CED8E0", "#FFF6DB", "#F9F0FD",
               "#FF0F0F", "#3288BD", "#B39CD0", "white"]

plt.bar(index, alloy_stack, bar_width,
        color=back_colors[0], label=labels[0], edgecolor="#000000", zorder=2, linewidth=linewidth,)
plt.bar([i + bar_width for i in index],
        as_preload, bar_width, color=back_colors[1], label=labels[1], edgecolor="#000000", zorder=2, linewidth=linewidth,)
plt.bar([i + 2*bar_width for i in index],
        faasm, bar_width, color=back_colors[2], label=labels[2], edgecolor="#000000", zorder=2, linewidth=linewidth,)

# 在柱子中心位置显示数据
# for i in index:
#     plt.text(i + 0 * bar_width, 10 ** (math.log10(alloy_stack[i])/4),
#              str(alloy_stack[i]), ha='center', rotation=90)
#     plt.text(i + 1 * bar_width, 10 ** (math.log10(faasm[i])/4),
#              str(faasm[i]), ha='center', rotation=90)


plt.xlabel('Data Size')
plt.ylabel('Latency (us)')
# plt.title('Performance Comparison')
plt.xticks([i + 1 * bar_width for i in index], categories)
plt.yscale('log')  # 设置纵坐标为对数坐标轴
plt.legend(framealpha=0, loc=(0., 1.11), ncol=3)

as_thr     = [9.73, 155.7 , 1851  , 8421  , 9377.2]
as_pre_thr = [98.2, 1350.2, 5753.7, 9195.4, 9696.9]
faasm_thr  = [0.56, 8.18  , 122.2 , 528   , 727   ]

ax2 = plt.twinx()
ax2.set_ylabel("Throughput (MB/s)")
ax2.plot([i + 1*bar_width for i in range(len(as_thr))], as_thr, label=labels[0], 
    color=back_colors[3], linewidth=linewidth, marker="s")
ax2.plot([i + 1*bar_width for i in range(len(as_pre_thr))], as_pre_thr, label=labels[1], 
    color=back_colors[4], linewidth=linewidth, marker="s")
ax2.plot([i + 1*bar_width for i in range(len(faasm_thr))], faasm_thr, label=labels[2], 
    color=back_colors[5], linewidth=linewidth, marker="s")

ax2.set_yscale('log')
ax2.set_ylim([0.01, 30000])
labelss = ax2.legend(framealpha=0, loc=(0., 1.01), ncol=3,).get_texts()
[label.set_color('white') for label in labelss]

plt.grid(ls="--", zorder=1)
plt.savefig('/home/yjn/Downloads/sec6_transfer.pdf')
# plt.show()
