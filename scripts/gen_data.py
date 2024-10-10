#!/usr/bin/python3

import os


def gen_word_count(file_num: int, total_size: int):
    from faker import Faker

    fake = Faker()
    one_size = int(total_size / file_num)

    for i in range(file_num):
        file_name = f'./image_content/fake_data_{i}.txt'
        with open(file_name, 'w') as f:
            while True:
                f.write(fake.text(10_000))

                if os.stat(file_name).st_size > one_size:
                    break


def gen_parallel_sort(file_num: int, total_size: int):
    import random

    one_size = int(total_size / file_num)
    for i in range(file_num):
        file_name = f'./image_content/sort_data_{i}.txt'
        current_size = 0
        with open(file_name, 'w') as f:
            while current_size < one_size:
                text = ' '.join(str(random.randint(0, 1000000)) for i in range(10))
                text += '\n'
                f.write(text)
                current_size += len(text.encode('utf-8'))  # 更新当前文件大小


if __name__ == "__main__":
    gen_parallel_sort(1, 60 * 1024)
