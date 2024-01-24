#!python3

import subprocess
import os

# pidof gateway
gateway_pid = 12612
# pidof faasnetes
faasnetes_pid = 12686
# sudo docker inspect de8ecce0c5ed | grep -i Pid
fwatchdog_pid = 8092
# cat /proc/8092/task/8092/children
func_pid = 8126

pids = [gateway_pid, faasnetes_pid, fwatchdog_pid, func_pid]
proc_name = ["gateway", "faasnetes", "fwatchdog", "flask"]


def get_utime_and_stime(pid: int) -> (int, int):
    procfs_file = f"/proc/{pid}/stat"
    with open(procfs_file, "r") as f:
        content = f.read()

    fields = content.split(" ")
    utime, stime = int(fields[13]), int(fields[14])
    return (utime, stime)


def get_switch_counts(pid) -> (int, int):
    voluntary_switches_sum = 0
    involuntary_switches_sum = 0

    # 构建 /proc/<pid>/task 目录路径
    task_dir = f'/proc/{pid}/task'

    # 检查 /proc/<pid>/task 是否存在
    if os.path.exists(task_dir):
        # 获取所有子目录（对应任务）
        tasks = os.listdir(task_dir)

        # 遍历每个子目录
        for task in tasks:
            task_dir_path = os.path.join(task_dir, task)

            # 构建 /proc/<pid>/task/<task>/sched 文件路径
            sched_file_path = os.path.join(task_dir_path, 'sched')

            # 检查文件是否存在
            if os.path.exists(sched_file_path):
                try:
                    # 读取 nr_voluntary_switches 和 nr_involuntary_switches 字段
                    with open(sched_file_path, 'r') as sched_file:
                        for line in sched_file:
                            if line.startswith("nr_voluntary_switches"):
                                voluntary_switches_sum += int(line.split()[-1])
                            elif line.startswith("nr_involuntary_switches"):
                                involuntary_switches_sum += int(
                                    line.split()[-1])
                except Exception as e:
                    print(f"Error reading {sched_file_path}: {e}")

    return voluntary_switches_sum, involuntary_switches_sum


times1 = [get_utime_and_stime(pid) for pid in pids]
switch1 = [get_switch_counts(pid) for pid in pids]

result = subprocess.run(['python3', 'client.py'],
                        stdout=subprocess.PIPE, text=True, check=True)
print(result.stdout.strip())

times2 = [get_utime_and_stime(pid) for pid in pids]
switch2 = [get_switch_counts(pid) for pid in pids]

for i in range(len(pids)):
    print(proc_name[i], f"utime: {(times2[i][0]-times1[i][0])*10}ms",
          f"stime: {(times2[i][1]-times1[i][1])*10}ms", f"voluntary_switches: {switch2[i][0] - switch1[i][0]}")
