#!/bin/bash

sudo ip tuntap add name tap-1 mode tap user "$USER" \
  && sudo ip link set tap-1 up \
  && sudo ip addr add 192.168.69.100/24 dev tap-1 \
  && sudo iptables -t nat -A POSTROUTING -s 192.168.69.0/24 -j MASQUERADE \
  && sudo sysctl net.ipv4.ip_forward=1 \
  && sudo iptables -A FORWARD -i tap-1 -s 192.168.69.0/24 -j ACCEPT \
  && sudo iptables -A FORWARD -o tap-1 -d 192.168.69.0/24 -j ACCEPT