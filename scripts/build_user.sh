#!/bin/bash

if [ $# -gt 0 ]; then
    feature_arg="--features $1"
else
    feature_arg=""
fi

find user -name 'Cargo.toml' | xargs -I {} bash -c "cargo build $feature_arg --manifest-path {}"