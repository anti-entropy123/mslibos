import threading
import multiprocessing as mp
import psutil
import json
import time

from mpkmemalloc import *

def worker_func(start_time):
    now = time.time()
    print(now, now - start_time)

class MyThread(threading.Thread):
    def __init__(self, func, args=()):
        threading.Thread.__init__(self)
        self.func = func
        self.args = args

    def run(self):
        self.result = self.func(*self.args)

    def recv(self):
        try:
            return self.result
        except Exception as e:
            print("exception: ", e)
            return None

def runt(f, event):
	pkey_thread_mapper()
	result = f(event)
	pymem_reset()
	return result
    
def main():
	start = time.time()

	pymem_setup_allocators(0)

	psutil.Process().cpu_affinity([0])

	p = MyThread(runt, args=(worker_func, start))

	p.start()
	p.join()

if __name__ == "__main__":
    main()