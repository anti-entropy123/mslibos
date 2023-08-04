#!/bin/bash

find user -name 'Cargo.toml' | xargs -n 1 echo cargo clippy --manifest-path | bash