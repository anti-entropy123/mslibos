#!/bin/bash

sudo ip link set tap0 down \
  && sudo ip tuntap del dev tap0 mode tap