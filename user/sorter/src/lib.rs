#![no_std]

use alloc::{format, string::String, vec::Vec};

use ms_std::{
    args,
    prelude::*,
    time::{SystemTime, UNIX_EPOCH},
};
use ms_std_proc_macro::FaasData;

#[derive(Default, FaasData)]
struct Reader2Sorter {
    #[cfg(feature = "pkey_per_func")]
    raw_data: heapless::String<{ 110 * 1024 * 1024 }>,
    #[cfg(not(feature = "pkey_per_func"))]
    raw_data: String,
}

#[derive(Default, FaasData)]
struct VecArg {
    #[cfg(feature = "pkey_per_func")]
    array: heapless::Vec<u32, { 20 * 1024 * 1024 }>,
    #[cfg(not(feature = "pkey_per_func"))]
    array: Vec<u32>,
}

#[cfg(feature = "pkey_per_func")]
#[derive(Default, FaasData)]
struct Pivots {
    array: heapless::Vec<u32, 10>,
}

#[no_mangle]
pub fn main() -> Result<()> {
    println!(
        "com_start1: {}",
        SystemTime::now().duration_since(UNIX_EPOCH).as_micros() as f64 / 1000000f64
    );
    let my_id = args::get("id").unwrap();
    let sorter_num: usize = {
        let n = args::get("sorter_num").unwrap();
        n.parse().unwrap()
    };
    let merger_num: usize = {
        let n = args::get("merger_num").unwrap();
        n.parse().unwrap()
    };

    let input: DataBuffer<Reader2Sorter> =
        DataBuffer::from_buffer_slot(format!("input-part-{}", my_id)).unwrap();

    println!(
        "sorter: input-part-{}, input length={}",
        my_id,
        input.raw_data.len()
    );

    let mut buffer: DataBuffer<VecArg> =
        DataBuffer::with_slot(format!("sorter-resp-part-{}", my_id));
    for num in input.raw_data.split(',') {
        let num = num.trim();
        if num.is_empty() {
            continue;
        }
        let num = num
            .parse()
            .map_err(|_| format!("parse Int failed, num={}", num))?;
        buffer.array.push(num);
    }

    buffer.array.sort();

    if my_id.eq("0") {
        // let mut pivots: DataBuffer<VecArg> = ;
        let pivots: Vec<_> = (0..merger_num - 1)
            .map(|i| {
                let idx = (i + 1) * buffer.array.len() / merger_num;
                if idx >= buffer.array.len() {
                    println!("idx: {}, len: {}", idx, buffer.array.len());
                }
                buffer.array[idx]
            })
            .collect();

        for i in 0..sorter_num {
            #[cfg(not(feature = "pkey_per_func"))]
            {
                let mut pivots_buffer: DataBuffer<VecArg> =
                    DataBuffer::with_slot(format!("pivots-{}", i));
                pivots_buffer.array = pivots.clone();
            }
            #[cfg(feature = "pkey_per_func")]
            {
                let mut pivots_buffer: DataBuffer<Pivots> =
                    DataBuffer::with_slot(format!("pivots-{}", i));
                pivots_buffer.array = heapless::Vec::from_slice(&pivots).expect("pivots vec error");
            }
        }
    }
    println!(
        "com_end1: {}",
        SystemTime::now().duration_since(UNIX_EPOCH).as_micros() as f64 / 1000000f64
    );
    Ok(().into())
}
