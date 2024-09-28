#!/bin/bash

declare -A test_groups=(
    [faasflow_test]="long_chain map_reduce pass_complex_args"
    [file_test]="simple_file" # mmap_file 
    [run_dylib_test]="base_config"
    [should_panic_test]="should_panic"
    [wasm_test]="tinywasm tinywasm_py tinywasm_pass_args"
    # [network_test]="simple_http"
)

passed_count=0
declare -A results

for group in "${!test_groups[@]}"; do
    names=${test_groups[$group]}
    
    for name in $names; do
        if cargo run -- --files "isol_config/$name.json"; then
            # 如果命令正常退出，记录通过结果
            results[$name]="passed"
            ((passed_count++)) # 增加通过计数
        else
            # 如果命令失败，记录失败结果
            results[$name]="failed"
        fi
    done
done

# 输出所有测试结果
echo "Test Results:"
for name in "${!results[@]}"; do
    status=${results[$name]}
    if [ "$status" == "passed" ]; then
        echo -e "\033[32m$name: $status\033[0m"
    else
        echo -e "\033[31m$name: $status\033[0m"
    fi
done

total_tests=${#results[@]}

# 检查通过计数与总测试数量是否相等
if [ "$passed_count" -ne "$total_tests" ]; then
    echo "Error: Not all tests passed!"
    exit 1  # 返回失败状态
fi