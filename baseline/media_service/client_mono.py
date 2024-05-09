import requests
import random
import string
import json
import time
import concurrent.futures

batch_size = 1000
con_num = 16


def gen_random_string(i):
    choices = string.ascii_letters + string.digits
    return "".join([choices[random.randint(0, len(choices)-1)] for j in range(i)])


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


def mono(_):
    review = generate_data()
    print("gen review =", review)
    start = time.time()
    requests.post(
        "http://127.0.0.1:32331/function/media-service-faastlane", review).text

    return int((time.time()-start)*1000)


def batch_invoke():
    results = []
    start_time = time.time()
    with concurrent.futures.ThreadPoolExecutor(max_workers=con_num) as executor:
        resps = executor.map(mono, [None for i in range(batch_size)])

    end_time = time.time()
    print(f"mono mode, cost {time.time()-start_time} s")
    print("qps:", batch_size/(end_time-start_time))
    return results


if __name__ == '__main__':
    # batch_invoke()
    print("mono mode, cost", mono(()))
