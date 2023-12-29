#!python3

import requests
import json
import time
import concurrent.futures
from typing import List

func_name = "alu-flask-remote"
loop_time = 80_000
batch_size = 1000
con_num = 64

def invoke_func(func_name: str, data: dict) -> requests.Response:
    resp = requests.request(
        "GET", f'http://localhost:32331/function/{func_name}',
        headers={
            'mslibos-trace': f'invoke:{str(round(time.time()*1000_000))}'},
        data=json.dumps(data).encode()
    )
    resp.headers['mslibos-trace'] += ',finish:' + \
        str(round(time.time()*1000_000))

    return resp


def batch_invoke():
    alu_res = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=con_num) as executor:
        resps = executor.map(
            lambda data: invoke_func(func_name, data),
            [{'loop_time': loop_time} for i in range(batch_size)]
        )

        for resp in resps:
            alu_res.append(
                {'trace': resp.headers['mslibos-trace'], 'body': resp.text})

    end_time = time.time()
    return alu_res


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
        func_info[func_label] = [
            round(resp['body'][func_label]*1000, 3) for resp in resps]

    print(json.dumps(trace_info))
    print(json.dumps(func_info))


if __name__ == '__main__':
    resps = batch_invoke()
    display_breakdown(resps[10:])
