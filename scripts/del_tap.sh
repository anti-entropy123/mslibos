#!/bin/bash

sudo ip link set tap-1 down \
  && sudo ip tuntap del dev tap-1 mode tap