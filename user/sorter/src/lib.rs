#![no_std]

use alloc::{collections::BTreeMap, format, string::String, vec::Vec};

use ms_std::prelude::*;
use ms_std_proc_macro::FaasData;

#[derive(Default, FaasData)]
struct Reader2Sorter {
    raw_data: String,
}

#[derive(Default, FaasData)]
struct VecArg {
    array: Vec<u32>,
}

#[no_mangle]
pub fn main(args: &BTreeMap<String, String>) -> Result<()> {
    let my_id = &args["id"];
    let merger_num: usize = {
        let n = &args["merger_num"];
        n.parse().unwrap()
    };

    let input: DataBuffer<Reader2Sorter> =
        DataBuffer::from_buffer_slot(format!("input-part-{}", my_id)).unwrap();

    let mut array: DataBuffer<VecArg> =
        DataBuffer::with_slot(format!("sorter-resp-part-{}", my_id));
    for num in input.raw_data.split(',') {
        array.array.push(
            num.trim()
                .parse()
                .map_err(|_| format!("parse Int failed, num={}", num))?,
        )
    }

    array.array.sort();
    if my_id.eq("0") {
        // let mut pivots: DataBuffer<VecArg> = ;
        let pivots: Vec<_> = (0..merger_num - 1)
            .map(|i| {
                let idx = (i + 1) * array.array.len() / merger_num;
                array.array[idx]
            })
            .collect();

        for i in 0..merger_num {
            let mut pivots_buffer: DataBuffer<VecArg> =
                DataBuffer::with_slot(format!("pivots-{}", i));
            pivots_buffer.array = pivots.clone();
        }
    }

    Ok(().into())
}
