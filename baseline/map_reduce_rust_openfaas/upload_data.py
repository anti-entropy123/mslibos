#!python3

from minio import Minio

BUCKET_NAME = "parallel-sort-input"

minioClient = Minio("node-7:9331", access_key="admin123",
                    secret_key="admin123", secure=False)

if not minioClient.bucket_exists(BUCKET_NAME):
    minioClient.make_bucket(BUCKET_NAME)

for i in range(5):
    local_path = f"/home/yjn/rust_project/mslibos/input-part-{i}.txt"
    minioClient.fput_object(
        BUCKET_NAME, object_name=f"part-{i}", file_path=local_path)
