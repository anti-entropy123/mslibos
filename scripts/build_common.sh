#!/bin/bash

cargo build --manifest-path common_service/"$1"/Cargo.toml && \
    cp common_service/"$1"/target/debug/lib"$1".so ./target/debug