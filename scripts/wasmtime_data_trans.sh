#!/bin/bash

# 定义执行次数
EXECUTIONS=10

# 初始化变量来累加 total_dur 的值
total_dur_sum=0

# 循环执行十次
for (( i=1; i<=EXECUTIONS; i++ ))
do
    echo "Running iteration $i..."

    output=$(cargo run --release -- --preload --files ./isol_config/wasmtime_trans_data.json 2>&1)
    
    numbers=($(echo "$output" | grep -o '[0-9]\+\.[0-9]\+'))

    # 获取最后两个数字
    num1=${numbers[-2]}
    num2=${numbers[-1]}

    # 计算差值
    difference=$(echo "$num2 - $num1" | bc)

    # 累加 total_dur 的值
    total_dur_sum=$(echo "$total_dur_sum + $difference" | bc)

    # 打印结果
    echo "Difference: $difference"
done


# 计算平均值
average_total_dur=$(echo "scale=3; $total_dur_sum * 1000 / $EXECUTIONS" | bc)
echo "Average Total Dur (ms): $average_total_dur"