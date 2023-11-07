from minio import Minio

minioClient = Minio("10.244.4.131:9000", access_key="admin123",
                    secret_key="admin123", secure=False)

for i in range(5):
    local_path = "/mnt/minio/data-500m/part_%d" % (i+1)
    minioClient.fput_object(
        "data-500m", object_name="part-%d" % i, file_path=local_path)
