#!/bin/bash

find user -name 'Cargo.toml' | xargs -n 1 echo cargo build --manifest-path | bash