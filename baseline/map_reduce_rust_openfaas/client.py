#!python3

import json
import time
import subprocess
import sys

mapper_name = "mapper-rust"
reducer_name = "reducer-rust"
# mapper_name = "wc-mapper"
# reducer_name = "wc-reducer"
gateway = "node-7:32331"
# gateway = "localhost:31119"

mapper_num = int(sys.argv[1])
reducer_num = mapper_num


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
    start = time.time()
    mapper_res, reducer_res = workflow()
    # mapper_res: [
    #   '{"comp_time":651,"count_num":1935,"read_time":132,"store_time":19}',
    #   '{"comp_time":654,"count_num":1935,"read_time":142,"store_time":13}',
    #   '{"comp_time":639,"count_num":1935,"read_time":151,"store_time":12}',
    #   '{"comp_time":620,"count_num":1935,"read_time":124,"store_time":15}',
    #   '{"comp_time":635,"count_num":1935,"read_time":148,"store_time":13}']
    # reducer_res [
    #   '{"comp_time":5,"read_time":14,"store_time":17}',
    #   '{"comp_time":5,"read_time":14,"store_time":17}',
    #   '{"comp_time":1,"read_time":14,"store_time":12}',
    #   '{"comp_time":1,"read_time":14,"store_time":20}',
    #   '{"comp_time":5,"read_time":13,"store_time":18}']
    mapper_res = [json.loads(item) for item in mapper_res]
    reducer_res = [json.loads(item) for item in reducer_res]

    total = 1000*(time.time()-start)
    print(f"total cost time: {total}ms", )

    read_input_time = sum([item['read_time']
                          for item in mapper_res]) / len(mapper_res)

    im_data_latency_tx = 0
    im_data_latency_rx = 0
    comp_mapper = 0
    comp_reducer = 0
    for i in range(len(mapper_res)):
        im_data_latency_tx += mapper_res[i]["store_time"]
        im_data_latency_rx += reducer_res[i]["read_time"] + \
            reducer_res[i]["store_time"]
        comp_mapper += mapper_res[i]["comp_time"]
        comp_reducer += reducer_res[i]["comp_time"]

    im_data_latency_rx /= len(mapper_res)
    im_data_latency_tx /= len(mapper_res)
    comp_mapper /= len(mapper_res)
    comp_reducer /= len(reducer_res)

    print(f"read input data: {read_input_time}ms")
    print(
        f"intermediate data transfer: {im_data_latency_tx+im_data_latency_rx}ms")
    print(f"comp: {comp_mapper+comp_reducer}ms")
    print(f"other: {total - read_input_time - (im_data_latency_tx+im_data_latency_rx) - (comp_mapper+comp_reducer)}ms", )

    print("===Map Info===", mapper_res)
    # get_res_info(mapper_res)
    print("===Reduce Info===", reducer_res)
    # get_res_info(reducer_res)
