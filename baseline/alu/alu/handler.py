from multiprocessing import Process
import random

defaultLoopTime = 10000000
defaultParallelIndex = 100


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
    per_times = int(defaultLoopTime / defaultParallelIndex)
    threads = []

    com_start = time.time()
    for i in range(defaultParallelIndex):
        t = Process(target=doAlu, args=(per_times,))
        threads.append(t)

    for i in range(defaultParallelIndex):
        threads[i].start()
    for i in range(defaultParallelIndex):
        threads[i].join()

    return json.dumps({'comp_time': time.time() - com_start})


if __name__ == '__main__':
    handle(None)
