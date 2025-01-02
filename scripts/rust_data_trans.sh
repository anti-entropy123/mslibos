#!/bin/bash

# 定义执行次数
EXECUTIONS=10

if [ -f "./user/func_a/Cargo.toml" ]; then
    cargo clean --manifest-path ./user/func_a/Cargo.toml
fi

if [ -f "./user/func_b/Cargo.toml" ]; then  
    cargo clean --manifest-path ./user/func_b/Cargo.toml
fi

cargo build --features mpk --manifest-path ./user/func_a/Cargo.toml --release
cargo build --features mpk --manifest-path ./user/func_b/Cargo.toml --release

# 初始化变量来累加 total_dur 的值
total_dur_sum=0
total_rate_sum=0

# 循环执行十次
for (( i=1; i<=EXECUTIONS; i++ ))
do
    echo "Running iteration $i..."

    # 运行项目并提取 "total_dur(ms)" 的值
    # output=$(cargo run --release --features mpk -- --preload --metrics all --files ./isol_config/pass_complex_args.json 2>&1)
    output=$(cargo run --release --features mpk -- --metrics all --files ./isol_config/pass_complex_args.json 2>&1)
    total_dur=$(echo "$output" | grep -o '"total_dur(ms)": [0-9.]*' | awk -F': ' '{print $2}')

    # 提取 trans data time、total_size 和 transfer rate
    # trans_data_time=$(echo "$output" | grep -o 'trans data time: [0-9.]*µs' | awk -F': ' '{print $2}' | awk -F'µs' '{print $1}')
    trans_data_time=$(echo "$output" | grep -o 'trans data time: [0-9.]*[µm]s' | awk -F': ' '{print $2}' | awk '{
        if ($1 ~ /µs$/) {
            print $1
        } else if ($1 ~ /ms$/) {
            gsub(/ms$/, "", $1)
            print $1 * 1000 "µs"
        }
    }' | awk -F'µs' '{print $1}')
    total_size=$(echo "$output" | grep -o 'total_size: [0-9]* Bytes' | awk -F': ' '{print $2}' | awk -F' Bytes' '{print $1}')
    transfer_rate=$(echo "$output" | grep -o 'transfer rate: [0-9]* MB/s' | awk -F': ' '{print $2}' | awk -F' MB/s' '{print $1}')

    # 累加 total_dur 的值
    total_dur_sum=$(echo "$total_dur_sum + $trans_data_time" | bc)

    # 累加 total_rate 的值
    total_rate_sum=$(echo "$total_rate_sum + $transfer_rate" | bc)

    # 打印结果
    echo "$trans_data_time $total_size $transfer_rate"
done

# 计算平均值
average_total_dur=$(echo "scale=3; $total_dur_sum / $EXECUTIONS" | bc)
echo "Average Total Dur (us): $average_total_dur"
average_total_rate=$(echo "scale=3; $total_rate_sum / $EXECUTIONS" | bc)
echo "Average Total Dur (MB/s): $average_total_rate"
