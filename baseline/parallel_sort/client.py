#!python3

import json
import time
import concurrent.futures
import requests
from typing import List
import sys

sorter_name = "sorter-rust"
splitter_name = "splitter-rust"
merger_name = "merger-rust"

sorter_num = int(sys.argv[1])
merger_num = sorter_num

session = requests.sessions.Session()


def invoke_func(func_name: str, data: dict) -> requests.Response:
    resp = session.request(
        "GET", f'http://node-7:32331/function/{func_name}',
        # headers={
        #     'mslibos-trace': f'invoke:{str(round(time.time()*1000_000))}'},
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
            sorter_res.append({'body': resp.text})

    # sorter_end = time.time()

    splitter_res = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=sorter_num) as executor:
        resps = executor.map(
            lambda data: invoke_func(splitter_name, data),
            [{"input_name": "rust-sorter-resp",
                "input_part": i} for i in range(sorter_num)]
        )

        for resp in resps:
            splitter_res.append({'body': resp.text})

    # splitter_end = time.time()

    merger_res = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=merger_num) as executor:
        resps = executor.map(
            lambda data: invoke_func(merger_name, data),
            [{"input_name": "rust-splitter-resp",
                "input_part": i, "sorter_num": sorter_num} for i in range(merger_num)]
        )

        for resp in resps:
            merger_res.append({'body': resp.text})

    # merger_end = time.time()

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
    start = time.time()
    sorter_res, splitter_res, merger_res = workflow()
    sorter_res = [json.loads(item['body']) for item in sorter_res]
    splitter_res = [json.loads(item['body']) for item in splitter_res]
    merger_res = [json.loads(item['body']) for item in merger_res]

    total = 1000*(time.time()-start)

    # sorter: [{'body': '{"comp_time":145,"read_time":37,"store_time":955}'}, {'body': '{"comp_time":166,"read_time":33,"store_time":468}'}, {'body': '{"comp_time":143,"read_time":38,"store_time":935}'}]
    # splitter: [{'body': '{"comp_time":57,"read_time":36,"store_time":856}'}, {'body': '{"comp_time":45,"read_time":41,"store_time":901}'}, {'body': '{"comp_time":42,"read_time":37,"store_time":543}'}]
    # merger: [{'body': '{"comp_time":66,"read_time":42,"store_time":419}'}, {'body': '{"comp_time":75,"read_time":44,"store_time":963}'}, {'body': '{"comp_time":72,"read_time":43,"store_time":996}'}]
    
    read_input_time = sum([item["read_time"] for item in sorter_res])/len(sorter_res)
    comp_time = 0
    im_data_time = 0
    for i in range(len(sorter_res)):
        comp_time += sorter_res[i]["comp_time"] + splitter_res[i]["comp_time"] + merger_res[i]["comp_time"] 
        im_data_time += sorter_res[i]["store_time"] + splitter_res[i]["read_time"] + splitter_res[i]["store_time"]  + merger_res[i]["read_time"] + merger_res[i]["store_time"]

    comp_time /= len(sorter_res)
    im_data_time /= len(sorter_res)

    print(f"total cost time: { total }ms")
    print(f"read input data: {read_input_time}ms")
    print(
        f"intermediate data transfer: {im_data_time}ms")
    print(f"comp: {comp_time}ms")
    print(f"other: {total - read_input_time - im_data_time - comp_time}ms", )
    
    print(sorter_res)
    print(splitter_res)
    print(merger_res)
