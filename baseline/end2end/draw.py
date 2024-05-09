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
fig = plt.figure(figsize=(7, 4.5), dpi=300)

plt.subplots_adjust(hspace=0.05, wspace=None, top=0.8, left=0.15, right=0.95, bottom=0.2)

# 数据
categories = ['WordCount', 'ParallelSort',]
alloystack = [1136, 1016,]
openfaas = [963, 16661,]
# faastlane  = [500        , 500           ,]

labels = ["AlloyStack", "OpenFaas",
          # "Faastlane"
          ]

# 绘制柱状图
bar_width = 0.4
index = range(len(categories))

back_colors = ["#CED8E0", "#FFF6DB", "#F9F0FD",
               "#D5E8D4", "white", "white", "white"]

plt.bar(index, alloystack, bar_width,
        color=back_colors[0], label=labels[0], edgecolor="#000000", zorder=2, linewidth=linewidth,)
plt.bar([i + bar_width for i in index],
        openfaas, bar_width, color=back_colors[1], label=labels[1], edgecolor="#000000", zorder=2, linewidth=linewidth,)
# plt.bar([i + 2 * bar_width for i in index],
#         faastlane, bar_width, color=back_colors[2], label=labels[2], edgecolor="#000000", zorder=2, linewidth=linewidth,)


# 在柱子中心位置显示数据
# for i in index:
#     plt.text(i + 0 * bar_width, 10 ** (math.log10(alloystack[i])/2), # 10 ** (math.log10(alloystack[i])/4),
#              str(alloystack[i]), ha='center', rotation=90)
#     plt.text(i + 1 * bar_width, 10 ** (math.log10(openfaas[i])/2),
#              str(openfaas[i]), ha='center', rotation=90)
#     plt.text(i + 2 * bar_width, 10 ** (math.log10(faastlane[i])/4),
#              str(faastlane[i]), ha='center', rotation=90)

plt.ylabel('Workflow End-to-End Time (ms)')
# plt.title('Performance Comparison')
plt.xticks([i + 0.5 * bar_width for i in index], categories)
plt.yscale('log')  # 设置纵坐标为对数坐标轴
plt.legend(framealpha=0.5, loc=(0.02, 1.01), ncol=3)
plt.grid(ls="--", zorder=1)
plt.savefig('/home/yjn/Downloads/sec6_end2end.pdf')
# plt.show()
