
import json
import io
import time
from minio import Minio
from mpkmemalloc import *

minio_client = Minio("minio-service.yasb-mapreduce-db.svc.cluster.local:9000",
                     access_key="admin123", secret_key="admin123", secure=False)

def mapper(req):
    """handle a request to the function
    Args:
        req (str): request body
    """
    event = json.loads(req)

    input_name = event["input_name"]
    input_part = int(event["input_part"])
    reduce_num = int(event["reduce_num"])

    read_start = time.time()

    input_object = minio_client.get_object(input_name, "part-%d" % input_part)
    input_data = input_object.data.decode("utf-8")
    print("------------------------------------------------------input_data_size%d" %
          len(input_data))
    read_end = time.time()

    counts = {}

    lines = input_data.split("\n")

    print("------------------------------------------------------line_size%d" % len(lines))
    for line in lines:
        words = line.strip().split(" ")
        for word in words:
            if word.isalpha():
                if word not in counts:
                    counts[word] = 0
                counts[word] = counts[word] + 1
    print("------------------------------------------------------count_size%d" % len(counts))
    shuffle = {}
    for i in range(reduce_num):
        shuffle[i] = ''

    for word, count in counts.items():
        reduce_id = hash(word) % reduce_num
        shuffle[reduce_id] = shuffle[reduce_id] + "%s:%d;" % (word, count)

    for i in range(reduce_num):
        if shuffle[i][-1] == ";":
            shuffle[i] = shuffle[i][:-1]

    com_end = time.time()

    for i in range(reduce_num):
        if not shuffle[i] == '':
            name = "%s:%s:%d:%d" % (input_name, APP, input_part, i)
            redis_client.set(name, shuffle[i])

    store_end = time.time()

    result = {
        "read_start": read_start,
        "read_end": read_end,
        "com_end": com_end,
        "store_end": store_end,
        
    }

    return json.dumps(result)

def reducer(req):
    """handle a request to the function
    Args:
        req (str): request body
    """
    event = json.loads(req)

    input_name = event["input_name"]
    input_num = int(event["input_num"])
    reduce_part = int(event["reduce_part"])

    read_start = time.time()

    raw_datas = []
    for i in range(input_num):
        name = "%s:%s:%d:%d" % (input_name, APP, i, reduce_part)
        if redis_client.exists(name):
            raw_data = redis_client.get(name)
            raw_datas.append(raw_data)

    read_end = time.time()

    counts = {}
    for raw_data in raw_datas:
        str_data = raw_data.decode("utf-8")
        pairs = [d.split(":") for d in str_data.split(";")]
        for pair in pairs:
            if pair[0] not in counts:
                counts[pair[0]] = 0
            counts[pair[0]] = counts[pair[0]] + int(pair[1])

    com_end = time.time()

    output = "\n".join(["%s:%d" % (k, v) for k, v in counts.items()])
    output_bucket = "%s-output" % APP

    if not minio_client.bucket_exists(output_bucket):
        minio_client.make_bucket(output_bucket)
    output_object = "part-%d" % reduce_part

    try:
        stat = minio_client.stat_object(output_bucket, output_object)
        minio_client.remove_object(output_bucket, output_object)
    except Exception as e:
        pass

    stream = io.BytesIO(output.encode())
    stream_size = stream.getbuffer().nbytes
    minio_client.put_object(output_bucket, output_object,
                            stream, length=stream_size)

    store_end = time.time()

    result = {
        "read_start": read_start,
        "read_end": read_end,
        "com_end": com_end,
        "store_end": store_end
    }

    return json.dumps(result)


def handle(req):
    """handle a request to the function
    Args:
        req (str): request body
    """

    return req
