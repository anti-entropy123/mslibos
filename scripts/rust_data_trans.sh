#!/bin/bash

# 定义执行次数
EXECUTIONS=10

if [ -f "./user/func_a/Cargo.toml" ]; then
    cargo clean --manifest-path ./user/func_a/Cargo.toml
fi

cargo build --manifest-path ./user/func_a/Cargo.toml --release

# 初始化变量来累加 total_dur 的值
total_dur_sum=0

# 循环执行十次
for (( i=1; i<=EXECUTIONS; i++ ))
do
    echo "Running iteration $i..."

    # 运行项目并提取 "total_dur(ms)" 的值
    # output=$(cargo run --release -- --preload --metrics all --files ./isol_config/pass_complex_args.json 2>&1)
    output=$(cargo run --release -- --preload --files ./isol_config/pass_complex_args.json 2>&1)
    
    total_dur=$(echo "$output" | grep 'opt_dur=' | cut -d'=' -f2)
    # 累加 total_dur 的值
    total_dur_sum=$(echo "$total_dur_sum + $total_dur" | bc)

    # 打印结果
    echo "$total_dur"
done


# 计算平均值
average_total_dur=$(echo "scale=6; $total_dur_sum / $EXECUTIONS / 1000" | bc)
echo "Average Total Dur (ms): $average_total_dur"