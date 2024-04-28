#!python3

import requests
import threading
import time
import subprocess
import signal


def mslibos_client():
    url = "http://node-7:8000/workflow?isol_name=never_stop"
    p = subprocess.Popen(["zsh", "-c", f"'curl {url}'"], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    time.sleep(0.1)
    p.send_signal(signal.SIGINT)
    print("stdout", p.stdout.read(), "stderr", p.stderr.read())


if __name__ == "__main__":
    tasks = []
    for i in range(10):
        print("task", i)
        time.sleep(3)
        # t = threading.Thread(target=mslibos_client)
        # tasks.append(t)
        # t.start()
        mslibos_client()

    # for t in tasks:
    #     t.join()
    time.sleep(1000)
