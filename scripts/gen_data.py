#!/usr/bin/python3

import os

workdir = "image_content"


def gen_word_count(file_num: int, total_size: int):
    from faker import Faker

    fake = Faker()
    Faker.seed(42)
    one_size = int(total_size / file_num)

    for i in range(file_num):
        file_name = f'{workdir}/fake_data_{i}.txt'
        with open(file_name, 'w', encoding='utf-8') as f:
            while True:
                f.write(fake.text(10_000))

                if os.stat(file_name).st_size > one_size:
                    break


def gen_parallel_sort(file_num: int, total_size: int):
    import random

    one_size = int(total_size / file_num)
    for i in range(file_num):
        file_name = f'{workdir}/sort_data_{i}.txt'
        current_size = 0
        with open(file_name, 'w', encoding='utf-8') as f:
            while current_size < one_size:
                text = ','.join(str(random.randint(0, 1000000))
                                for i in range(10))
                text += ','
                f.write(text)
                current_size += len(text.encode('utf-8'))  # 更新当前文件大小

            f.write('1')


if __name__ == "__main__":
    gen_parallel_sort(5, 1 * 1024 * 1024)
    # gen_word_count(5, 1 * 1024 * 1024)
