#!python3

import json
import time
import subprocess

mapper_name = "mapper-rust"
reducer_name = "reducer-rust"
# mapper_name = "wc-mapper"
# reducer_name = "wc-reducer"
gateway = "node-7:9331"
# gateway = "localhost:31119"
mapper_num = 5
reducer_num = 5


def invoke_func(func_name: str, data: dict,) -> subprocess.Popen:
    p = subprocess.Popen(
        ["faas-cli", "-g", gateway, "invoke", func_name], stdin=subprocess.PIPE, stdout=subprocess.PIPE, shell=False)

    p.stdin.write(json.dumps(data).encode())
    p.stdin.close()
    return p


def workflow():
    start = time.time()

    mapper_res = []
    mapper_handlers = []
    for i in range(mapper_num):
        req = {"input_name": "data-500m",
               "input_part": i, "reduce_num": reducer_num}
        handler = invoke_func(
            mapper_name, req)
        mapper_handlers.append(handler)

    for mapper in mapper_handlers:
        mapper.wait()
        mapper_res.append(mapper.stdout.read().decode())

    mapper_end = time.time()

    reducer_res = []
    reducer_handlers = []
    for i in range(reducer_num):
        req = {"input_name": "data-500m",
               "input_num": mapper_num, "reduce_part": i}
        handler = invoke_func(
            reducer_name, req)
        reducer_handlers.append(handler)

    for reducer in reducer_handlers:
        reducer.wait()
        reducer_res.append(reducer.stdout.read().decode())

    reducer_end = time.time()

    return mapper_res, reducer_res


if __name__ == "__main__":
    mapper_res, reducer_res = workflow()

    print("===Map Info===", mapper_res)
    # get_res_info(mapper_res)
    print("===Reduce Info===", reducer_res)
    # get_res_info(reducer_res)
