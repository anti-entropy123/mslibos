#!/bin/bash

(find user -name 'Cargo.toml' && find common_service -name 'Cargo.toml') \
     | xargs -n 1 echo cargo clippy --manifest-path \
     | bash