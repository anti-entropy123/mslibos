#!/bin/bash

feature_arg=""
release_flag=""

# 解析命令行参数
while [[ $# -gt 0 ]]; do
    case $1 in
        --release)
            release_flag="--release"
            shift
            ;;
        *)  # 处理features参数
            feature_arg="--features $1"
            shift
            ;;
    esac
done

find user -name 'Cargo.toml' \
    -not -path 'user/nn_conv/Cargo.toml' \
    -not -path 'user/mmap_file/Cargo.toml' \
    -not -path 'user/httpd/Cargo.toml' \
    -not -path 'user/should_panic/Cargo.toml' \
    -not -path 'user/load_all/Cargo.toml' \
    -not -path 'user/simple_http/Cargo.toml' \
    -not -path 'user/never_stop/Cargo.toml' \
    -not -path 'user/tinywasm*/Cargo.toml' \
    -not -path 'user/wasmtime*/Cargo.toml' | \
    xargs -I {} bash -c "cargo build $feature_arg --manifest-path {} $release_flag"