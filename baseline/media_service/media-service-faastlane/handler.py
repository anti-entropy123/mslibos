import json
import time
import threading
import random
import hashlib
import string
import sys

import pymongo
import memcache

upload_user_id_mongo_url = "mongodb://user-mongodb.movie-reviewing.svc.cluster.local:27017/"
upload_user_id_memcache_url = "user-memcached.movie-reviewing.svc.cluster.local:11211"
upload_user_id_mc = memcache.Client([upload_user_id_memcache_url])

upload_movie_id_mongo_url = "mongodb://movie-id-mongodb.movie-reviewing.svc.cluster.local:27017/"
upload_movie_id_memcache_url = "movie-id-memcached.movie-reviewing.svc.cluster.local:11211"
upload_movie_id_mc = memcache.Client([upload_movie_id_memcache_url])

from mpkmemalloc import *

def compose_review_handle(req):
    """handle a request to the function
    Args:
        req (str): request body
    """
    arguments = ["title", "text", "username", "password", "rating"]

    start = time.time()

    event = json.loads(req)

    response = {"time": {"compose-review": {"start_time": start}}}

    complete = False
    try:
        for arg in arguments:
            if event[arg] == '':
                break
        complete = True
    except Exception as e:
        pass

    if complete:
        response["body"] = event
    else:
        response["body"] = "Incomplete arguments"

    response["time"]["compose-review"]["end_time"] = time.time()
    return json.dumps(response)


def upload_user_id_handle(req):
    """handle a request to the function
    Args:
        req (str): request body
    """
    start = time.time()

    myclient = pymongo.MongoClient(upload_user_id_mongo_url)
    mydb = myclient['user']
    mycol = mydb["user"]

    event = json.loads(req)
    body = ''
    username = ''
    response = {"time": {"upload-user-id": {"start_time": start}}}
    try:
        username = event["body"]["username"]
    except Exception as e:
        body = 'Incomplete arguments'

    if username:
        user_id = -1
        mmc_user_id = upload_user_id_mc.get(username+":user_id")
        if mmc_user_id != None:
            user_id = mmc_user_id
            body = {"user_id": user_id}
        else:
            myquery = {"username": username}
            mydoc = mycol.find(myquery)
            print(mydoc)
            if mydoc.count() > 0:
                it = mydoc.next()
                user_id = it["user_id"]
                body = {"user_id": user_id}
                upload_user_id_mc.set(username+":user_id", user_id)
            else:
                body = 'No user ' + username

    response["body"] = body

    response["time"]["upload-user-id"]["end_time"] = time.time()
    return json.dumps(response)


def hash_key(title):
    m = hashlib.md5()
    m.update(title.encode("utf-8"))
    return m.hexdigest()


def upload_movie_id_handle(req):
    """handle a request to the function
    Args:
        req (str): request body
    """
    start = time.time()

    myclient = pymongo.MongoClient(upload_movie_id_mongo_url)
    mydb = myclient['movie-id']
    mycol = mydb["movie-id"]

    event = json.loads(req)
    body = ''
    title = ''
    title_hash = ''
    rating = -1
    response = {"time": {"upload-movie-id": {"start_time": start}}}
    try:
        title = event["body"]["title"]
        title_hash = hash_key(title)
        rating = int(event["body"]["rating"])
    except Exception as e:
        response['body'] = 'Incomplete arguments'

    if title and rating > -1:
        movie_id = ''
        mmc_movie_id = upload_movie_id_mc.get(title_hash)
        if mmc_movie_id != None:
            movie_id = mmc_movie_id
            response["body"] = {"movie_id": movie_id, 'rating': rating}
        else:
            myquery = {"title": title}
            mydoc = mycol.find(myquery)
            if mydoc.count() > 0:
                it = mydoc.next()
                movie_id = it["movie_id"]
                response["body"] = {"movie_id": movie_id, 'rating': rating}
                upload_movie_id_mc.set(title_hash, movie_id)
            else:
                response["body"] = 'No movie ' + title

    response["time"]["upload-movie-id"]["end_time"] = time.time()
    return json.dumps(response)


def mr_upload_text_handle(req):
    """handle a request to the function
    Args:
        req (str): request body
    """
    start = time.time()

    event = json.loads(req)

    response = {"time": {"mr-upload-text": {"start_time": start}}}
    text = ''
    try:
        text = event["body"]["text"]
        response["time"].update(event["time"])
    except Exception as e:
        response["body"] = 'Incomplete arguments'

    if text:
        response["body"] = {"text": text}

    response["time"]["mr-upload-text"]["end_time"] = time.time()

    return json.dumps(response)


def gen_random_digits(i):
    return "".join(random.sample(string.digits, i))


def mr_upload_unique_id_handle(req):
    """handle a request to the function
    Args:
        req (str): request body
    """
    start = time.time()

    event = json.loads(req)

    response = {"time": {"mr-upload-unique-id": {"start_time": start}}}

    machine_id = gen_random_digits(2)
    timestamp = str(int(time.time()*1000) - 1514764800000)[-11:]
    index_id = gen_random_digits(3)
    review_id = machine_id + timestamp + index_id

    response["body"] = {"review_id": review_id}
    response["time"]["mr-upload-unique-id"]["end_time"] = time.time()
    return json.dumps(response)


def mr_compose_and_upload_handle(req):
    """handle a request to the function
    Args:
        req (str): request body
    """
    start = time.time()

    events = json.loads(req)
    events = [json.loads(event) for event in events]
    print("mr_compose_and_upload_handle: events=", events, file=sys.stderr)

    body = {}
    response = {"time": {"mr-compose-and-upload": {"start_time": start}}}

    if len(events) < 4:
        body = 'Incomplete arguments'
        print("no")
    else:
        print("ok")
        try:
            for event in events:
                body.update(event["body"])
                response["time"].update(event["time"])
            body["timestamp"] = int(time.time()*1000)
        except Exception as e:
            body = 'Incomplete arguments'

    print("mr_compose_and_upload_handle: body=", body, file=sys.stderr)
    response["body"] = body
    response["time"]["mr-compose-and-upload"]["end_time"] = time.time()
    return json.dumps(response)


def store_review_handle(req):
    """handle a request to the function
    Args:
        req (str): request body
    """
    start = time.time()

    myclient = pymongo.MongoClient(
        "mongodb://review-storage-mongodb.movie-reviewing.svc.cluster.local:27017/")
    mydb = myclient["review"]
    mycol = mydb["review"]

    event = json.loads(req)
    print("store_review_handle: event=", event, file=sys.stderr)
    response = {"time": {"store-review": {"start_time": start}}}

    arguments = set(["review_id", "timestamp", "user_id",
                    "movie_id", "text", "rating"])
    if set(event["body"].keys()) == arguments:
        response["time"].update(event["time"])
        mycol.insert_one(event["body"])
    else:
        response['body'] = 'Incomplete arguments'

    response["time"]["store-review"]["end_time"] = time.time()
    return json.dumps(response)


def upload_user_review_handle(req):
    """handle a request to the function
    Args:
        req (str): request body
    """
    start = time.time()

    myclient = pymongo.MongoClient(
        "mongodb://user-review-mongodb.movie-reviewing.svc.cluster.local:27017/")
    mydb = myclient["user-review"]
    mycol = mydb["user-review"]

    event = json.loads(req)
    response = {"time": {"upload-user-review": {"start_time": start}}}

    try:
        user_id = event["body"]["user_id"]
        review_id = event["body"]["review_id"]
        timestamp = event["body"]["timestamp"]

        myquery = {"user_id": user_id}
        mydoc = mycol.find(myquery)
        if mydoc.count() > 0:
            reviews = json.loads(mydoc.next()["reviews"])
            reviews.append((review_id, timestamp))
            reviews_update = {"$set": {"reviews": json.dumps(reviews)}}
            mycol.update_one(myquery, reviews_update)
        else:
            body = {"user_id": user_id, "reviews": json.dumps(
                [(review_id, timestamp)])}
            mycol.insert_one(body)
    except Exception as e:
        response['body'] = 'Incomplete arguments'

    response["time"]["upload-user-review"]["end_time"] = time.time()
    return json.dumps(response)


def upload_movie_review_handle(req):
    """handle a request to the function
    Args:
        req (str): request body
    """
    start = time.time()

    myclient = pymongo.MongoClient(
        "mongodb://movie-review-mongodb.movie-reviewing.svc.cluster.local:27017/")
    mydb = myclient["movie-review"]
    mycol = mydb["movie-review"]

    event = json.loads(req)
    response = {"time": {"upload-movie-review": {"start_time": start}}}

    try:
        movie_id = event["body"]["movie_id"]
        review_id = event["body"]["review_id"]
        timestamp = event["body"]["timestamp"]

        myquery = {"movie_id": movie_id}
        mydoc = mycol.find(myquery)
        if mydoc.count() > 0:
            reviews = json.loads(mydoc.next()["reviews"])
            reviews.append((review_id, timestamp))
            reviews_update = {"$set": {"reviews": json.dumps(reviews)}}
            mycol.update_one(myquery, reviews_update)
        else:
            body = {"movie_id": movie_id, "reviews": json.dumps(
                [(review_id, timestamp)])}
            mycol.insert_one(body)
    except Exception as e:
        response['body'] = 'Incomplete arguments'

    response["time"]["upload-movie-review"]["end_time"] = time.time()
    return json.dumps(response)


def invoke_func(func, data, l):
    res = runt(func, data)
    l.append(res)


class MyThread(threading.Thread):
    def __init__(self, func, args=()):
        threading.Thread.__init__(self)
        self.func = func
        self.args = args

    def run(self):
        self.result = self.func(*self.args)

    def recv(self):
        try:
            return self.result
        except Exception:
            return None

def runt(f, event):
	pkey_thread_mapper()
	result = f(event)
	pymem_reset()
	return result
    

def handle(req):
    """handle a request to the function
    Args:
        req (str): request body
    """
    start = time.time()
    compose_res = compose_review_handle(req)

    parallel_res = []
    threads = []
    for func in [upload_user_id_handle, upload_movie_id_handle, mr_upload_text_handle, mr_upload_unique_id_handle]:
        t = MyThread(invoke_func, args=(
            func, compose_res, parallel_res))
        threads.append(t)

    for t in threads:
        t.start()

    for t in threads:
        t.join()

    compose_and_upload_res = mr_compose_and_upload_handle(
        json.dumps(parallel_res))

    parallel_res = []
    threads = []
    for func in [store_review_handle, upload_user_review_handle, upload_movie_review_handle]:
        t = MyThread(invoke_func, args=(
            func, compose_and_upload_res, parallel_res))
        threads.append(t)

    for t in threads:
        t.start()

    for t in threads:
        t.join()

    return f"cost time {(time.time() - start) * 1000} ms"
