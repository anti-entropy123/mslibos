#!python3

import json
import time
import subprocess

sorter_name = "sorter-rust"
splitter_name = "splitter-rust"
merger_name = "merger-rust"

sorter_num = 5
merger_num = 4


def invoke_func(func_name: str, data: dict,) -> subprocess.Popen:
    p = subprocess.Popen(
        ["/home/yjn/.local/bin/faas-cli", "-g", "localhost:32331", "invoke", func_name], stdin=subprocess.PIPE, stdout=subprocess.PIPE, shell=False)

    p.stdin.write(json.dumps(data).encode())
    p.stdin.close()
    return p


def workflow():
    start = time.time()

    sorter_res = []
    sorter_handlers = []
    for i in range(sorter_num):
        req = {"input_name": "parallel-sort-input",
               "input_part": i, "merger_num": merger_num}
        handler = invoke_func(
            sorter_name, req)
        sorter_handlers.append(handler)

    for sorter in sorter_handlers:
        sorter.wait()
        sorter_res.append(sorter.stdout.read().decode())

    sorter_end = time.time()

    splitter_res = []
    splitter_handlers = []
    for i in range(sorter_num):
        req = {"input_name": "rust-sorter-resp",
               "input_part": i}
        handler = invoke_func(
            splitter_name, req)
        splitter_handlers.append(handler)

    for splitter in splitter_handlers:
        splitter.wait()
        splitter_res.append(splitter.stdout.read().decode())

    splitter_end = time.time()

    merger_res = []
    merger_handlers = []
    for i in range(merger_num):
        req = {"input_name": "rust-splitter-resp",
               "input_part": i, "sorter_num": sorter_num}
        handler = invoke_func(
            merger_name, req)
        merger_handlers.append(handler)

    for merger in merger_handlers:
        merger.wait()
        merger_res.append(merger.stdout.read().decode())

    merger_end = time.time()

    return sorter_res, splitter_res, merger_res


if __name__ == "__main__":
    sorter_res, splitter_res, merger_res = workflow()

    print("===Sort Info===", sorter_res)
    # get_res_info(mapper_res)
    print("===Split Info===", splitter_res)
    # get_res_info(reducer_res)
    print("===Merge Info===", merger_res)
