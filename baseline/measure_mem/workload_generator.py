#!python3

import requests


def mslibos_client():
    url = "http://192.168.1.141:8000/workflow?isol_name=never_stop"
    r1 = requests.get(url)
    print("r1")
    r2 = requests.get(url)
    r3 = requests.get(url)

    print(r1.content, r2.content, r3.content)


if __name__ == "__main__":
    mslibos_client()
