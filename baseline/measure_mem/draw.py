import matplotlib.pyplot as plt
import matplotlib.font_manager
import matplotlib as mpl

matplotlib.rcParams["font.family"] = 'Helvetica'
matplotlib.rcParams['pdf.fonttype'] = 42
# matplotlib.rcParams['pdf.use14corefonts'] = True
matplotlib.rcParams['ps.fonttype'] = 42
plt.rcParams.update({'font.size': 11})

# fig = plt.figure(figsize=(4.65, 2.5), dpi=300)
fig = plt.figure(figsize=(4.25, 2.4), dpi=300)

plt.subplots_adjust(hspace=0.05, wspace=None, top=0.9,
                    bottom=0.15, left=0.18, right=0.98)

AS_overheads = [4029.333333, 6472, 8569.333333, 10645.33333,
                12702.66667, 14752, 16797.33333, 18864, 20928, 22974.66667, 25016]
AS_overheads = [(i-AS_overheads[0])/1000 for i in AS_overheads[1:]]

AS_http_overheads = [4021.333333, 8429.333333, 12498.66667, 16544.66667, 20605.33333,
                     24677.33333, 28744, 32801.33333, 36861.33333, 40897.33333, 44957.33333,]
AS_http_overheads = [(i-AS_http_overheads[0]) /
                     1000 for i in AS_http_overheads[1:]]

Faasm_overhead = [166740, 218818, 244458, 270642, 340040,
                  357606, 375172, 392730, 410290, 427888, 445454]
Faasm_overhead = [(i-Faasm_overhead[0])/1000 for i in Faasm_overhead[1:]]

Unikraft_overhead = [0, 12252, 24034.66667, 37029.33333, 49992,
                     61894.66667, 74785.33333, 86228, 98637.33333, 111421.3333, 123541.3333]
Unikraft_overhead = [(i-Unikraft_overhead[0]) /
                     1000 for i in Unikraft_overhead[1:]]

instance_nums = [i+1 for i in range(10)]

linewidth = 0.8
# markers = ["*", '♣\clubsuit', "P"]

# plt.plot(range(len(ASF_overheads)), ASF_overheads, label="ASF + S3",
#     color="#FF0F0F", linewidth=linewidth, marker="o")
plt.plot(range(len(AS_overheads)), AS_overheads, label="AS",
         color="#FF0F0F", linewidth=linewidth, marker="s")
plt.plot(range(len(AS_http_overheads)), AS_http_overheads, label="AS-http",
         color="#3288BD", linewidth=linewidth, marker="s")
plt.plot(range(len(Faasm_overhead)), Faasm_overhead, label="Faasm",
         color="#D11ED1", linewidth=linewidth, marker="s")
plt.plot(range(len(Unikraft_overhead)), Unikraft_overhead, label="Unikraft",
         color="#2F4858", linewidth=linewidth, marker="s")

# plt.text(1, ASF_overheads[2], "ASF + S3")
# plt.text(2, OpenFaas_overheads[2], "OpenFaaS + MinIO")

# plt.yscale("log")

plt.gca().xaxis.set_tick_params(pad=0.5, length=1)
plt.gca().yaxis.set_tick_params(pad=0.5, length=2)

plt.ylabel("Memory Footprint (MB)", fontsize=10, labelpad=0)
plt.xlabel("Num of Instances", fontsize=10, labelpad=0)
plt.xticks(range(len(instance_nums)), instance_nums)

plt.grid(ls="--", zorder=1)
plt.legend(loc=(0.01, 1.01), ncol=4, prop={'size': 8})

plt.savefig("/home/yjn/Downloads/sec6_mem.pdf")

plt.show()
