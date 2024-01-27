
import requests
import random
import string
import json
import time
import concurrent.futures
import threading

compose_review_url = "http://127.0.0.1:32331/function/compose-review"
upload_user_id_url = "http://127.0.0.1:32331/function/upload-user-id"

funcs = ["compose-review", "upload-user-id", ]

batch_size = 850
con_num = 16


def gen_random_string(i):
    choices = string.ascii_letters + string.digits
    return "".join([choices[random.randint(0, len(choices)-1)] for j in range(i)])


def post(url, data, l):
    res = requests.post(url, data).text
    l.append(res)
    # print(url, res)


movie_titles = []
with open("movie_titles.csv", "r") as f:
    movie_titles = f.readlines()


def generate_data():
    user_index = str(random.randint(1, 1000))
    review = {
        "username": "username_" + user_index,
        "password": "password_" + user_index,
        "title": movie_titles[random.randint(0, len(movie_titles)-1)].strip(),
        "rating": random.randint(0, 10),
        "text": gen_random_string(256)
    }
    # print(review)

    return json.dumps(review)


def workflow(_):
    review = generate_data()

    start = time.time()
    compose_res = requests.post(compose_review_url, data=review).text

    parallel_res = []
    threads = []
    for url in [upload_user_id_url,]:
        t = threading.Thread(target=post, args=(
            url, compose_res, parallel_res))
        threads.append(t)

    for t in threads:
        t.start()

    for t in threads:
        t.join()

    # print(parallel_res)


def batch_invoke():
    results = []
    start_time = time.time()
    with concurrent.futures.ThreadPoolExecutor(max_workers=con_num) as executor:
        resps = executor.map(workflow, [None for i in range(batch_size)])

        for resp in resps:
            results.append(
                {'trace': resp.headers['mslibos-trace'], 'X-Duration-Seconds': resp.headers['X-Duration-Seconds'], 'body': resp.text})

    end_time = time.time()
    print("qps:", batch_size/(end_time-start_time))
    return results


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
    batch_invoke()
