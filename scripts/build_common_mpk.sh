#!/bin/bash

if [ ! -n "$2" ] ;then
    build_mode="debug"
else
    build_mode="release"
fi

cargo build $2 --features mpk --manifest-path common_service/$1/Cargo.toml && \
    cp common_service/$1/target/$build_mode/lib"$1".so ./target/$build_mode
