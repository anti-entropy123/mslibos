#!python3

import requests
import threading
import time
import subprocess
import signal


instance_num = 30


def mslibos_client():
    url = "http://localhost:8000/workflow?isol_name=never_stop"
    requests.get(url)


if __name__ == "__main__":
    tasks = []
    for i in range(instance_num):
        print("task", i)
        time.sleep(0.2)
        t = threading.Thread(target=mslibos_client)
        tasks.append(t)
        t.start()

    for t in tasks:
        t.join()
