#!python3

import os


def gen_word_count():
    from faker import Faker

    fake = Faker()

    FILE_NUM = 8

    for i in range(FILE_NUM):
        file_name = f'fake_data_{i}.txt'
        with open(file_name, 'w') as f:
            while True:
                f.write(fake.text(10_000))

                if os.stat(file_name).st_size > 60_000_000:
                    break


def gen_parallel_sort():
    import random

    sorter_num = 5
    for i in range(sorter_num):
        file_name = f'input-part-{i}.txt'
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

                if os.stat(file_name).st_size > 60_000_000:
                    break


if __name__ == "__main__":
    gen_parallel_sort()
