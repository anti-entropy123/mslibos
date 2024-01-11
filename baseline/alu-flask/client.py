#!python3

import requests
import json
import time
import concurrent.futures
from typing import List
import random

func_name = "alu-flask-remote"
loop_time = 80_000
batch_size = 850  # 0
con_num = 16

payload_size = [0, 512, 4096]  # KB
payloads: List[str] = [''.join([str(random.randint(0, 9)) for i in range(size*1024)])
                       for size in payload_size]


def invoke_func(func_name: str, data: dict) -> requests.Response:
    resp = requests.request(
        "GET", f'http://localhost:32331/function/{func_name}',
        headers={
            'mslibos-trace': f'invoke:{str(round(time.time()*1000_000))}'},
        data=json.dumps(data).encode()
    )
    resp.headers['mslibos-trace'] += f',finish:{ str(round(time.time()*1000_000))}'

    return resp


def batch_invoke(payload: str):
    alu_res = []
    start_time = time.time()
    with concurrent.futures.ThreadPoolExecutor(max_workers=con_num) as executor:
        resps = executor.map(
            lambda data: invoke_func(func_name, data),
            [{'loop_time': loop_time, 'payload': payload}
                for i in range(batch_size)]
        )

        for resp in resps:
            alu_res.append(
                {'trace': resp.headers['mslibos-trace'], 'X-Duration-Seconds': resp.headers['X-Duration-Seconds'], 'body': resp.text})

    end_time = time.time()
    print("qps:", batch_size/(end_time-start_time))
    return alu_res


def breakdown(resps: list) -> str:
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

    # f',X-Duration-Useconds:{round(float(resp.headers["X-Duration-Seconds"]) * 1000_000)}'
    trace_info["X-Duration-Useconds"] = [
        round(float(resp["X-Duration-Seconds"]) * 1000) for resp in resps]

    func_info = {}
    for func_label in resps[0]['body']:
        func_info[func_label] = [
            round(resp['body'][func_label]*1000, 3) for resp in resps]

    return json.dumps(trace_info) + '\n' + json.dumps(func_info)


if __name__ == '__main__':
    for payload in payloads:
        resps = batch_invoke(payload)
        result = breakdown(resps[100:])
        with open(f"alu-flask-remote-{len(payload)//1024}kb.trace", 'w') as f:
            f.write(result)
