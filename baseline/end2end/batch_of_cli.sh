#!/bin/bash

batch_size=100
c_num=5

echo batch_size $batch_size c_num $c_num

for ((i=1;i<=$batch_size;i++))
do 
    echo $i
    python3 /home/yjn/rust_project/mslibos/baseline/map_reduce_rust_openfaas/client.py $c_num | grep total
    sleep 0.5
done