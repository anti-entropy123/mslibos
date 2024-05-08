#!/bin/bash

batch_size=100
c_num=1
# app_name=map_reduce_large
app_name=parallel_sort

echo batch_size $batch_size c_num $c_num

for ((i=1;i<=$batch_size;i++))
do 
    echo $i
    cargo run --release -- --files isol_config/${app_name}_c${c_num}.json --metrics total-dur 2>&1 | grep total_dur
    sleep 0.5
done