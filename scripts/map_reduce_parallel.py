#!/usr/bin/python3

import subprocess
import re

# 并行执行 ./scripts/rust_end_to_end.sh map_reduce c3


def test_map_reduce_parallel():
    import concurrent.futures

    def run_script(instance):
        command = f"./scripts/rust_end_to_end.sh map_reduce {instance}"
        result = subprocess.run(command, shell=True,
                                capture_output=True, text=True)
        return result.stdout, result.stderr

    instances = ['c3', 'c3', 'c3', 'c3', 'c3', 'c3', 'c3',
                 'c3', 'c3', 'c3']  # Add more instances as needed
    latencies = []

    with concurrent.futures.ThreadPoolExecutor() as executor:
        futures = [executor.submit(run_script, instance)
                   for instance in instances]
        for future in concurrent.futures.as_completed(futures):
            stdout, stderr = future.result()
            # 从 stdout 中提取 'Total Dur (ms): 37.825' 中的数字
            for line in stdout.splitlines():
                match = re.search(r'Total Dur \(ms\): (\d+\.\d+)', line)
                if match:
                    duration = float(match.group(1))
                    # print("Total Duration (ms):", duration)
                    latencies.append(duration)
            # print("Output:", stdout)
            if stderr:
                print("Error:", stderr)

    print("Latencies:", sorted(latencies))


if __name__ == '__main__':
    test_map_reduce_parallel()
