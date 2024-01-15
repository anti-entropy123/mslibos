#!/usr/bin/bash

echo 'compose_review upload_user_id upload_movie_id mr_upload_text mr_upload_unique mr_compose_and_upload store_review' | 
    xargs -n 1 printf 'user/%s/Cargo.toml\n' |
    xargs -n 1 cargo build $1 --no-default-features --features mock --manifest-path && \
cargo run $1 -- --files isol_config/media_service_no_libos.json --metrics all
