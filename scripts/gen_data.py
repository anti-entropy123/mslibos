#!python3

from faker import Faker
import os

fake = Faker()

FILE_NUM = 8

for i in range(FILE_NUM):
    file_name = f'fake_data_{i}.txt'
    with open(file_name, 'w') as f:
        while True:
            f.write(fake.text(10_000))

            if os.stat(file_name).st_size > 60_000_000:
                break
