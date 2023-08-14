import matplotlib.pyplot as plt
from matplotlib import rc

rc('mathtext', default='regular')

latency = [207, 174, 173, 196, 210, 198, 197]  # us
throughput = [20, 47, 71, 83, 97, 124, 145]  # MB/s
dats_sizes = [f'{i * 4}KB' for i in range(1, 8)]

fig = plt.figure()

ax = fig.add_subplot(111)
p_latency = ax.plot(dats_sizes, latency, '-', label='Latency (µs)')
ax.grid()
ax.set_xlabel("DataSize (KB)")
ax.set_ylabel("Latency (µs)")
ax.set_ylim(100, 500)

ax2 = ax.twinx()
p_throughtput = ax2.plot(dats_sizes, throughput, 'r',
                         label='Throughput (MB/s)')
ax2.set_ylabel("Throughput (MB/s)")

ps = p_latency+p_throughtput
labels = [p.get_label() for p in ps]

ax.legend(ps, labels, loc=0)

plt.show()
