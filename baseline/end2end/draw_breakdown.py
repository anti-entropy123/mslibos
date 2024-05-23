import matplotlib.pyplot as plt
import matplotlib as mpl
import numpy as np
import math
import matplotlib.font_manager
from matplotlib.patches import Patch

matplotlib.rcParams["font.family"] = 'Arial'
matplotlib.rcParams['pdf.fonttype'] = 42
matplotlib.rcParams['ps.fonttype'] = 42
plt.rcParams.update({'font.size': 18})

# 线宽
linewidth = 0.8
mpl.rcParams['hatch.linewidth'] = linewidth

# 数据
categories = ['WordCount', 'ParallelSort',]

# Map Reduce
as_mr_total = [600.36, 566.83, 623.77,]
as_mr_total = sum(as_mr_total) / len(as_mr_total)

as_mr_read_input = [200.33, 184.66, 223.0,]
as_mr_read_input = sum(as_mr_read_input)/len(as_mr_read_input) / as_mr_total

as_mr_comp = [359.33+11.33, 354.0+5.666, 369.66+12.66,]
as_mr_comp = sum(as_mr_comp)/len(as_mr_comp) / as_mr_total


of_mr_total = [609.95, 567.742, 531.11, 523.979,]
of_mr_total = sum(of_mr_total) / len(of_mr_total) / as_mr_total

of_mr_read_input = [80.666, 78.33, 76.333, 80.0,]
of_mr_read_input = sum(of_mr_read_input)/len(of_mr_read_input) / as_mr_total

of_mr_im_data = [30.6666, 34.0, 31.66666, 31.333,]
of_mr_im_data = sum(of_mr_im_data)/len(of_mr_im_data) / as_mr_total

of_mr_comp = [365.0, 341.666, 345.3333, 338.333,]
of_mr_comp = sum(of_mr_comp)/len(of_mr_comp) / as_mr_total


print(1, as_mr_read_input, as_mr_comp, of_mr_total,
      of_mr_read_input, of_mr_comp, of_mr_im_data)

# Parallel Sort
as_ps_total = [371.45, 342.69, 315.37,]
as_ps_total = sum(as_ps_total) / len(as_ps_total)

as_ps_read_input = [58.0, 62.66, 53.66,]
as_ps_read_input = sum(as_ps_read_input)/len(as_ps_read_input) / as_ps_total

as_ps_comp = [178.33+15.33+82.66+8.0, 168.0 +
              17.66+70.33+8.0, 149.66+14.0+63.66+8.0,]
as_ps_comp = sum(as_ps_comp)/len(as_ps_comp) / as_ps_total

of_ps_total = [2350.087, 2881.62, 2420.477, 3493.00, 2015.28,]
of_ps_total = sum(of_ps_total) / len(of_ps_total) / as_ps_total

of_ps_read_input = [33.6666, 33.666, 37., 33.0, 34.666,]
of_ps_read_input = sum(of_ps_read_input)/len(of_ps_read_input) / as_ps_total

of_ps_im_data = [1796.333, 2131.333, 1846.666, 2865.33, 1534.0,]
of_ps_im_data = sum(of_ps_im_data)/len(of_ps_im_data) / as_ps_total

of_ps_comp = [250.0, 250.0, 249.0, 249.333, 247.333,]
of_ps_comp = sum(of_ps_comp)/len(of_ps_comp) / as_ps_total

print(1, as_ps_read_input, as_ps_comp, of_ps_total,
      of_ps_read_input, of_ps_comp, of_ps_im_data)

labels = ["AlloyStack", "OpenFaaS",]
hatches = ['|', '/', '\\', 'x']

# 绘制柱状图
bar_width = 0.4
index = range(len(categories))

back_colors = ["#CED8E0", "#FFF6DB", "#F9F0FD",
               "#D5E8D4", "white", "white", "white"]

fig, axs = plt.subplots(1, 2, figsize=(7.5, 4))  # 2行3列

axs[0].set_xlim([-0.5, 1.5])
axs[0].set_xticks([0, 1], labels)
# axs[0].tick_params(axis='y', which='major', labelrotation=90)
axs[0].set_xlabel("Map Reduce", labelpad=14)
axs[0].set_ylabel("End-to-end (normalized)", labelpad=10)
axs[0].grid(ls="--", zorder=1)

# draw as mr
axs[0].bar(0, 1, bar_width,
           color=back_colors[0], edgecolor="#000000", zorder=2, linewidth=linewidth,)
axs[0].bar(0, as_mr_read_input, bar_width,
           color=back_colors[0], edgecolor="#000000", zorder=2, linewidth=linewidth, hatch=hatches[0])
axs[0].bar(0, as_mr_comp, bar_width, bottom=as_mr_read_input,
           color=back_colors[0], edgecolor="#000000", zorder=2, linewidth=linewidth, hatch=hatches[-1])

# draw of mr
cursor = 0
axs[0].bar(1, of_mr_total, bar_width,
           color=back_colors[1], edgecolor="#000000", zorder=2, linewidth=linewidth,)
axs[0].bar(1, of_mr_read_input, bar_width,
           color=back_colors[1], edgecolor="#000000", zorder=2, linewidth=linewidth, hatch=hatches[0])
cursor += of_mr_read_input
axs[0].bar(1, of_mr_comp, bar_width, bottom=cursor,
           color=back_colors[1], edgecolor="#000000", zorder=2, linewidth=linewidth, hatch=hatches[1])
cursor += of_mr_comp
axs[0].bar(1, of_mr_im_data, bar_width, bottom=cursor,
           color=back_colors[1], edgecolor="#000000", zorder=2, linewidth=linewidth, hatch=hatches[2])

axs[1].set_xlim([-0.5, 1.5])
axs[1].set_xticks([0, 1], labels)
axs[1].set_xlabel("Parallel Sort", labelpad=14)
axs[1].grid(ls="--", zorder=1)

# draw as ps
axs[1].bar(0, 1, bar_width,
           color=back_colors[0], edgecolor="#000000", zorder=2, linewidth=linewidth,)
axs[1].bar(0, as_ps_read_input, bar_width,
           color=back_colors[0], edgecolor="#000000", zorder=2, linewidth=linewidth, hatch=hatches[0])
axs[1].bar(0, as_ps_comp, bar_width, bottom=as_ps_read_input,
           color=back_colors[0], edgecolor="#000000", zorder=2, linewidth=linewidth, hatch=hatches[-1])


# draw of ps
cursor = 0
axs[1].bar(1, of_ps_total, bar_width,
           color=back_colors[1], edgecolor="#000000", zorder=2, linewidth=linewidth,)
axs[1].bar(1, of_ps_read_input, bar_width,
           color=back_colors[1], edgecolor="#000000", zorder=2, linewidth=linewidth, hatch=hatches[0])
cursor += of_ps_read_input
axs[1].bar(1, of_ps_comp, bar_width, bottom=cursor,
           color=back_colors[1], edgecolor="#000000", zorder=2, linewidth=linewidth, hatch=hatches[1])
cursor += of_ps_comp
axs[1].bar(1, of_ps_im_data, bar_width, bottom=cursor,
           color=back_colors[1], edgecolor="#000000", zorder=2, linewidth=linewidth, hatch=hatches[2])

plt.tight_layout(rect=[0, 0, 1, 0.95])

legend_elements = [
    #     Patch(facecolor=back_colors[0], edgecolor='black', label='AS'),
    Patch(facecolor='white', edgecolor='black',
          hatch='||', label='read input'),
    Patch(facecolor='white', edgecolor='black',
          hatch='//', label='computation'),
    Patch(facecolor='white', edgecolor='black',
          hatch='\\\\', label='transfer data'),
    #     Patch(facecolor='white', edgecolor='black', hatch='xx', label='both'),
]
axs[0].legend(handles=legend_elements, frameon=False,
              loc=(-0.03, 1.03),
              ncol=3,
              handletextpad=0.5,  # 图例项符号和文本之间的间距
              # labelspacing=1.5,  # 图例项之间的垂直间距
              # borderpad=1,  # 图例边框和内容之间的间距
              handlelength=1  # 图例项符号的长度
            )

# plt.show()
plt.savefig(f"/home/yjn/Downloads/sec6_end2end_breakdown.pdf")
