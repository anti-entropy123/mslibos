
import requests
import random
import string
import json
import time
import concurrent.futures
import threading

compose_review_url = "http://127.0.0.1:32331/function/compose-review"
upload_user_id_url = "http://127.0.0.1:32331/function/upload-user-id"
upload_movie_id_url = "http://127.0.0.1:32331/function/upload-movie-id"
mr_upload_text_url = "http://127.0.0.1:32331/function/mr-upload-text"
mr_upload_unique_id_url = "http://127.0.0.1:32331/function/mr-upload-unique-id"
mr_compose_and_upload_url = "http://127.0.0.1:32331/function/mr-compose-and-upload"
store_review_url = "http://127.0.0.1:32331/function/store-review"
upload_user_review_url = "http://127.0.0.1:32331/function/upload-user-review"
upload_movie_review_url = "http://127.0.0.1:32331/function/upload-movie-review"

funcs = ["compose-review", "upload-user-id", "upload-movie-id", "mr-upload-text", "mr-upload-unique-id",
         "mr-compose-and-upload", "store-review", "upload-user-review", "upload-movie-review"]

batch_size = 1000
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
    for url in [upload_user_id_url, upload_movie_id_url, mr_upload_text_url, mr_upload_unique_id_url]:
        t = threading.Thread(target=post, args=(
            url, compose_res, parallel_res))
        threads.append(t)

    for t in threads:
        t.start()

    for t in threads:
        t.join()

    compose_and_upload_res = requests.post(
        mr_compose_and_upload_url, data=json.dumps(parallel_res)).text

    parallel_res = []
    threads = []
    for url in [store_review_url, upload_user_review_url, upload_movie_review_url]:
        t = threading.Thread(target=post, args=(
            url, compose_and_upload_res, parallel_res))
        threads.append(t)

    for t in threads:
        t.start()

    for t in threads:
        t.join()

    return int((time.time()-start)*1000)


def batch_invoke():
    results = []
    start_time = time.time()
    with concurrent.futures.ThreadPoolExecutor(max_workers=con_num) as executor:
        resps = executor.map(workflow, [None for i in range(batch_size)])

    end_time = time.time()
    print(f"trace workflow mode, cost {time.time()-start_time} s")
    print("qps:", batch_size/(end_time-start_time))
    return results


if __name__ == '__main__':
    batch_invoke()
