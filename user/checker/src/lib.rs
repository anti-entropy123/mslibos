#![no_std]
#![feature(is_sorted)]

use alloc::{format, vec::Vec};

use ms_std::{args, prelude::*};
use ms_std_proc_macro::FaasData;

#[derive(Default, FaasData)]
struct VecArg {
    array: Vec<u32>,
}

#[no_mangle]
pub fn main() -> Result<()> {
    let merger_num: u32 = {
        let m = args::get("merger_num").unwrap();
        m.parse().unwrap()
    };

    let merged_results = (0..merger_num).map(|idx| {
        DataBuffer::<VecArg>::from_buffer_slot(format!("merge_result_{}", idx)).unwrap()
    });

    let mut last_max = u32::MIN;
    let mut counter = 0;
    for array in merged_results {
        counter += array.array.len();
        if !array.array.is_sorted() {
            Err("sort result is wrong")?
        }

        if let Some(first) = array.array.first() {
            if *first < last_max {
                Err("sort result is wrong")?
            }
            last_max = *first
        }
    }

    println!("sort result is ok, total sort {} numbers", counter);
    Ok(().into())
}
