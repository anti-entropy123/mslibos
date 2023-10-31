#!python3
import time

read_start = time.time()
with open('../../fs_images/fake_data_0.txt') as f: 
    lines = f.read().splitlines()

read_end = time.time()
print("read take %s ms" % ((read_end - read_start) * 1000))

counts = {}

for line in lines:
        words = line.strip().split(" ")
        for word in words:
            if word.isalpha():
                if word not in counts:
                    counts[word] = 0 
                counts[word] = counts[word] + 1

print("comp take %s ms" % ((time.time() - read_end) * 1000))
print("has computed %d words" % len(counts))