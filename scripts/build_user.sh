#!/bin/bash

find user -name 'Cargo.toml' -print0 | xargs -n 1 echo cargo build "$1" --manifest-path | bash