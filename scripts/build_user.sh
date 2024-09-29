#!/bin/bash

if [ $# -gt 0 ]; then
    feature_arg="--features $1"
else
    feature_arg=""
fi

find user -name 'Cargo.toml' \
    -not -path 'user/nn_conv/Cargo.toml' \
    -not -path 'user/mmap_file/Cargo.toml' \
    -not -path 'user/httpd/Cargo.toml' \
    -not -path 'user/should_panic/Cargo.toml' \
    -not -path 'user/load_all/Cargo.toml' \
    -not -path 'user/simple_http/Cargo.toml' \
    -not -path 'user/never_stop/Cargo.toml' | \
    xargs -I {} bash -c "cargo build $feature_arg --manifest-path {}"