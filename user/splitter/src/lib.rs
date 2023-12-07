#![no_std]
use alloc::{collections::BTreeMap, format, string::String, vec::Vec};

use ms_std::{prelude::*, println};
use ms_std_proc_macro::FaasData;

#[derive(Default, FaasData)]
struct VecArg {
    array: Vec<u32>,
}

#[no_mangle]
pub fn main(args: &BTreeMap<String, String>) -> Result<()> {
    let my_id = &args["id"];

    let numbers: DataBuffer<VecArg> =
        DataBuffer::from_buffer_slot(format!("sorter-resp-part-{}", my_id)).unwrap();
    let pivots: DataBuffer<VecArg> =
        DataBuffer::from_buffer_slot(format!("pivots-{}", my_id)).unwrap();

    let partitions = split_numbers(&numbers.array, &pivots.array);
    for (idx, partition) in partitions.iter().enumerate() {
        let mut part: DataBuffer<VecArg> =
            DataBuffer::with_slot(format!("splitter-{}-resp-part-{}", my_id, idx));
        part.array = partition.clone();
    }

    println!(
        "len of numbers is {}, has split into {} parts: {:?}",
        numbers.array.len(),
        partitions.len(),
        partitions
            .iter()
            .map(|part| part.len())
            .collect::<Vec<usize>>()
    );

    Ok(().into())
}

fn split_numbers(numbers: &Vec<u32>, pivots: &Vec<u32>) -> Vec<Vec<u32>> {
    let mut result = Vec::new();
    let mut current_start = 0;

    for &pivot in pivots {
        let mut current_partition = Vec::new();

        for &num in &numbers[current_start..] {
            if num < pivot {
                current_partition.push(num);
            } else {
                break;
            }
        }

        result.push(current_partition.clone());
        current_start += current_partition.len();

        if current_start >= numbers.len() {
            break;
        }
    }

    // Add the remaining numbers as the last partition
    result.push(numbers[current_start..].to_vec());

    result
}
