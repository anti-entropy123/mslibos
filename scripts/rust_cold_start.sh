#!/bin/bash

# 定义执行次数
EXECUTIONS=10

# 初始化变量来累加 total_dur 的值
total_dur_sum=0

if [ -f "./user/hello_world/Cargo.toml" ]; then
    cargo clean --manifest-path ./user/hello_world/Cargo.toml
fi

cargo build --manifest-path ./user/hello_world/Cargo.toml --release

# 循环执行十次
for (( i=1; i<=EXECUTIONS; i++ ))
do
    echo "Running iteration $i..."

    # 运行项目并提取 "total_dur(ms)" 的值
    output=$(cargo run --release -- --metrics all --files ./isol_config/base_config.json 2>&1)
    # output=$(cargo run --release -- --preload --metrics all --files ./isol_config/base_config.json 2>&1)
    total_dur=$(echo "$output" | grep -o '"total_dur(ms)": [0-9.]*' | awk -F': ' '{print $2}')

    # 保留三位小数，并进行四舍五入
    total_dur_rounded=$(printf "%.3f\n" "$total_dur")

    # 累加 total_dur 的值
    total_dur_sum=$(echo "$total_dur_sum + $total_dur_rounded" | bc)

    # 打印结果
    echo "Total Dur (ms): $total_dur_rounded"
done

# 计算平均值
average_total_dur=$(echo "scale=3; $total_dur_sum / $EXECUTIONS" | bc)
echo "Average Total Dur (ms): $average_total_dur"