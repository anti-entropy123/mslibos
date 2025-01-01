#![no_std]
use alloc::{format, vec::Vec};

use ms_std::{args, prelude::*,time::{SystemTime, UNIX_EPOCH}};
use ms_std_proc_macro::FaasData;

#[derive(Default, FaasData)]
struct VecArg {
    array: Vec<u32>,
}

#[no_mangle]
pub fn main() -> Result<()> {
    println!("com_start3: {}", SystemTime::now().duration_since(UNIX_EPOCH).as_micros() as f64 / 1000000f64);
    let my_id = args::get("id").unwrap();
    let sorter_num: u32 = {
        let m = args::get("sorter_num").unwrap();
        m.parse().unwrap()
    };

    let partitions: Vec<DataBuffer<VecArg>> = (0..sorter_num)
        .map(|idx| {
            DataBuffer::from_buffer_slot(format!("splitter-{}-resp-part-{}", idx, my_id)).unwrap()
        })
        .collect();

    let mut merged_result: DataBuffer<VecArg> =
        DataBuffer::with_slot(format!("merge_result_{}", my_id));

    merged_result.array = merge_partitions(partitions.iter().map(|buffer| &buffer.array).collect());
    // println!("merged_result: {:?}", merged_result);
    println!("com_end3: {}", SystemTime::now().duration_since(UNIX_EPOCH).as_micros() as f64 / 1000000f64);
    Ok(().into())
}

fn merge_partitions(partitions: Vec<&Vec<u32>>) -> Vec<u32> {
    let mut result = Vec::new();

    let mut indices: Vec<usize> = partitions.iter().map(|_| 0).collect();

    loop {
        let mut min_value = core::u32::MAX;
        let mut min_partition = None;

        for (i, &index) in indices.iter().enumerate() {
            if index < partitions[i].len()
                && *partitions[i].get(index).unwrap_or(&u32::MAX) < min_value
            {
                min_value = partitions[i][index];
                min_partition = Some(i);
            }
        }

        match min_partition {
            Some(partition_idx) => {
                result.push(min_value);
                indices[partition_idx] += 1;
            }
            None => break,
        }
    }

    result
}
