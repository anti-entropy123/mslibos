#!python3

from minio import Minio

minioClient = Minio("node-7:9331", access_key="admin123",
                    secret_key="admin123", secure=False)

for i in range(5):
    local_path = "/home/yjn/Downloads/fake_data_%d.txt" % (i)
    minioClient.fput_object(
        "data-500m", object_name="part-%d" % i, file_path=local_path)
