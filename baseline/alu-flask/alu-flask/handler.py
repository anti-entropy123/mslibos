from multiprocessing import Process
import random
import time
import json

defaultLoopTime = 10_000

# copy from ServerlessBench.
# https://github.com/SJTU-IPADS/ServerlessBench/blob/master/Testcase1-Resource-efficiency/code/alu.py


def doAlu(times):
    a = random.randint(10, 100)
    b = random.randint(10, 100)
    temp = 0
    for i in range(times):
        if i % 4 == 0:
            temp = a + b
        elif i % 4 == 1:
            temp = a - b
        elif i % 4 == 2:
            temp = a * b
        else:
            temp = a / b


def handle(req):
    """handle a request to the function
    Args:
        req (str): request body
    """
    req: dict = json.loads(req)
    loopTime = req.get('loop_time') or defaultLoopTime

    com_start = time.time()
    doAlu(loopTime)

    return json.dumps({'comp_time': time.time() - com_start})
