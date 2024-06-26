#!/usr/bin/python3

import os


def gen_word_count(file_num: int, total_size: int):
    from faker import Faker

    fake = Faker()
    one_size = int(total_size / file_num)

    for i in range(file_num):
        file_name = f'fake_data_{i}.txt'
        with open(file_name, 'w') as f:
            while True:
                f.write(fake.text(10_000))

                if os.stat(file_name).st_size > one_size:
                    break


def gen_parallel_sort(file_num: int, total_size: int):
    import random

    one_size = int(total_size / file_num)
    for i in range(file_num):
        file_name = f'part-{i}'
        first = True

        with open(file_name, 'w') as f:
            while True:
                text = ','.join([str(random.randint(0, 1000000))
                                for i in range(200)])
                if not first:
                    text = ',' + text
                else:
                    first = False

                f.write(text)

                if os.stat(file_name).st_size > one_size:
                    break


if __name__ == "__main__":
    gen_parallel_sort(1, 5*1024*1024)
