#!/bin/bash

if [ ! -d "./fs_images" ]; then
  mkdir ./fs_images
fi

dd if=/dev/zero of=./fs_images/fatfs.img bs=1M seek=128 count=0 && \
    mkfs.fat ./fs_images/fatfs.img