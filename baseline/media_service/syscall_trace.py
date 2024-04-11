#!/usr/bin/python3

import subprocess
import threading
import os
import signal
import sys
import time

# pidof gateway
gateway_pid = 3649
# pidof faasnetes
faasnetes_pid = 3727

# sudo docker inspect $(sudo docker ps | grep mr-upload-text | head -n 1 | awk '{print $1}') | grep -i pid
# ps auxf | grep -A 1 32642
workflow_procs = [{
    "name": "compose-review",
    "watchdog": 6140,
    # "func": 6171,
}, {
    "name": "upload-user-id",
    "watchdog": 6710,
    # "func": 6740,
}, {
    "name": "upload-movie-id",
    "watchdog": 32642,
    # "func": 32672,
}, {
    "name": "mr-upload-text",
    "watchdog": 5153,
    # "func": 5185,
}, {
    "name": "mr-upload-unique-id",
    "watchdog": 11057,
    # "func": 11089,
}, {
    "name": "mr-compose-and-upload",
    "watchdog": 6830,
    # "func": 6860,
}, {
    "name": "store-review",
    "watchdog": 11254,
    # "func": 11297,
}, {
    "name": "upload-user-review",
    "watchdog": 15797,
    # "func": 15830,
}, {
    "name": "upload-movie-review",
    "watchdog": 18791,
    # "func": 18817,
},]

# sudo docker ps | grep media-service | grep watchdog | awk '{print $1}' | sudo xargs -n 1 docker inspect  | grep -i '"pid"'
mono_watchdog = [
    20466, 19055, 17546, 15957, 14438, 13002, 11565, 10024, 8497,
]

child_processes = []
threads = []


def run_strace(name: str, pid: str):
    child = subprocess.Popen(["strace", "-f", "-c", "-p", pid],
                             stderr=subprocess.PIPE, text=True,)

    child_processes.append(child.pid)

    _, stderr = child.communicate()
    # print(name, ":\n", stderr)
    with open(f'./{name}.log', "w") as f:
        f.write(stderr)


def trace_workflow_mode():
    for proc in workflow_procs:
        t1 = threading.Thread(target=run_strace, args=(
            f'{proc["name"]}-watchdog', str(proc["watchdog"]),))
        # t2 = threading.Thread(target=run_strace, args=(
        #     f'{proc["name"]}-func', str(proc["func"]),))
        threads.append(t1)
        # threads.append(t2)

    [t.start() for t in threads]
    time.sleep(0.1)
    if len(child_processes) < len(workflow_procs)+2:
        sys.exit("strace failed?")

    start = time.time()
    result = subprocess.run(['python3', 'client.py'],
                            stdout=subprocess.PIPE, text=True, check=True)
    print(f"trace workflow mode, cost {time.time()-start} s")

    print(result.stdout.strip())


def trace_mono_mode():
    for idx,pid in enumerate(mono_watchdog):
        t = threading.Thread(target=run_strace, args=(
            f'mr-mono-{idx}', str(pid),))
        threads.append(t)

    [t.start() for t in threads]
    time.sleep(0.1)
    if len(child_processes) < len(mono_watchdog)+2:
        sys.exit("strace failed?")

    start = time.time()
    result = subprocess.run(['python3', 'client_mono.py'],
                            stdout=subprocess.PIPE, text=True, check=True)
    print(f"trace mono mode, cost {time.time()-start} s")

    print(result.stdout.strip())


if __name__ == '__main__':
    if os.geteuid() != 0:
        sys.exit("use sudo!")

    workflow = True

    t = threading.Thread(target=run_strace, args=(
        f'gateway', str(gateway_pid),))
    threads.append(t)

    t = threading.Thread(target=run_strace, args=(
        f'faas-netes', str(faasnetes_pid),))
    threads.append(t)

    if workflow:
        trace_workflow_mode()
    else:
        trace_mono_mode()

    for child_pid in child_processes:
        os.kill(child_pid, signal.SIGINT)

    [t.join() for t in threads]
