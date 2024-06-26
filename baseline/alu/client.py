#!python3

import requests
import json
import time
import concurrent.futures
from typing import List


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
    with concurrent.futures.ThreadPoolExecutor(max_workers=10) as executor:
        batch_size = 50
        resps = executor.map(
            lambda data: invoke_func("alu", data),
            [{'loop_time': 1_000_000} for i in range(batch_size)]
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
        func_info[func_label] = [resp['body'][func_label] for resp in resps]

    print(trace_info)
    print(func_info)


if __name__ == '__main__':
    resps = batch_invoke()
    display_breakdown(resps[10:])
