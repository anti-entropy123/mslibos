#!python3

import json
import time
import concurrent.futures
import requests
from typing import List

sorter_name = "sorter-rust"
splitter_name = "splitter-rust"
merger_name = "merger-rust"

sorter_num = 5
merger_num = 4

session = requests.sessions.Session()


def invoke_func(func_name: str, data: dict) -> requests.Response:
    resp = session.request(
        "GET", f'http://10.244.1.227:8080/function/{func_name}',
        headers={
            'mslibos-trace': f'invoke:{str(round(time.time()*1000_000))}'},
        data=json.dumps(data).encode()
    )

    return resp


def workflow():
    start = time.time()

    sorter_res = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=sorter_num) as executor:
        resps = executor.map(
            lambda data: invoke_func(sorter_name, data),
            [{"input_name": "parallel-sort-input",
              "input_part": i, "merger_num": merger_num} for i in range(sorter_num)]
        )

        for resp in resps:
            sorter_res.append(
                {'trace': resp.headers['mslibos-trace'], 'body': resp.text})

    sorter_end = time.time()

    splitter_res = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=sorter_num) as executor:
        resps = executor.map(
            lambda data: invoke_func(splitter_name, data),
            [{"input_name": "rust-sorter-resp",
                "input_part": i} for i in range(sorter_num)]
        )

        for resp in resps:
            splitter_res.append(
                {'trace': resp.headers['mslibos-trace'], 'body': resp.text})

    splitter_end = time.time()

    merger_res = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=merger_num) as executor:
        resps = executor.map(
            lambda data: invoke_func(merger_name, data),
            [{"input_name": "rust-splitter-resp",
                "input_part": i, "sorter_num": sorter_num} for i in range(merger_num)]
        )

        for resp in resps:
            merger_res.append(
                {'trace': resp.headers['mslibos-trace'], 'body': resp.text})

    merger_end = time.time()

    return sorter_res, splitter_res, merger_res


def display_breakdown(resps: list):
    trace_info = {}
    for resp in resps:
        resp['body'] = json.loads(resp['body'])

        trace_str = resp['trace']
        resp['trace'] = {}

        for items in trace_str.split(','):
            kv = items.split(':')
            resp['trace'][kv[0].strip()] = int(
                kv[1].strip())

        base = resp['trace']['invoke']
        for label, val in resp['trace'].items():
            resp['trace'][label] = round((val - base) / 1000, 3)

    for trace_label in resps[0]['trace']:
        trace_info[trace_label] = [resp['trace'][trace_label]
                                   for resp in resps]

    func_info = {}
    for func_label in resps[0]['body']:
        func_info[func_label] = [resp['body'][func_label] for resp in resps]

    print(trace_info)
    print(func_info)


if __name__ == "__main__":
    sorter_res, splitter_res, merger_res = workflow()

    print("===Sort Info===")
    display_breakdown(sorter_res)

    # get_res_info(mapper_res)
    print("===Split Info===")
    display_breakdown(splitter_res)

    # get_res_info(reducer_res)
    print("===Merge Info===")
    display_breakdown(merger_res)
