#!python3

import matplotlib.pyplot as plt
from matplotlib import rc

rc('mathtext', default='regular')

latency = [38, 37, 38, 38, 37, 35]  # us
throughput = [540, 665, 755, 863, 997, 1171]  # MB/s
dats_sizes = [f'{i * 4}KB' for i in range(5, 11)]

fig = plt.figure()

ax = fig.add_subplot(111)
p_latency = ax.plot(dats_sizes, latency, '-', label='Latency (µs)')
ax.grid()
ax.set_xlabel("DataSize (KB)")
ax.set_ylabel("Latency (µs)")
ax.set_ylim(30, 70)

ax2 = ax.twinx()
p_throughtput = ax2.plot(dats_sizes, throughput, 'r',
                         label='Throughput (MB/s)')
ax2.set_ylabel("Throughput (MB/s)")

ps = p_latency+p_throughtput
labels = [p.get_label() for p in ps]

ax.legend(ps, labels, loc=0)

plt.show()
