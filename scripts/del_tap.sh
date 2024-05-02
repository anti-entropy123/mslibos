#!/bin/bash

if [ -z "$1" ] ;then
    idx="1"
else
    idx=$1
fi


sudo ip link set tap-"$idx" down \
  && sudo ip tuntap del dev tap-"$idx" mode tap