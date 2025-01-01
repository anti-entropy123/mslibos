#![no_std]

extern crate alloc;

// use alloc::vec::Vec;
use alloc::{format, string::String};
// use ms_std::{
//     agent::{DataBuffer, FaaSFuncResult as Result},
//     println,
//     time::{SystemTime, UNIX_EPOCH},
// };
use ms_std::agent::{DataBuffer, FaaSFuncResult as Result};
use ms_std_proc_macro::FaasData;

#[derive(Default, FaasData)]
struct Reader2Arraysum {
    raw_data: String,
}

const DATA_SIZE: usize = 1024 * 1024 * 1;

#[allow(clippy::result_unit_err)]
#[no_mangle]
pub fn main() -> Result<()> {
    // let my_id = args::get("id").unwrap();
    // let array_num: u64 = args::get("array_num")
    //     .expect("missing arg reducer_num")
    //     .parse()
    //     .unwrap_or_else(|_| panic!("bad arg, array_num={}", args::get("array_num").unwrap()));
    // println!("get id {}", my_id);
    // if array_num == 0 {
    //     let mut buffer: DataBuffer<Reader2Arraysum> = DataBuffer::with_slot(format!("part-{}-{}", array_num, my_id));
    //     buffer.raw_data = "a".repeat(DATA_SIZE);
    // } else if array_num == 9 {
    //     let reader: DataBuffer<Reader2Arraysum> = DataBuffer::from_buffer_slot(format!("part-{}-{}", array_num-1, my_id)).expect("missing input data.");
    //     let result = reader.raw_data.clone();
    // } else {
    //     let mut buffer: DataBuffer<Reader2Arraysum> = DataBuffer::with_slot(format!("part-{}-{}", array_num, my_id));
    //     let reader: DataBuffer<Reader2Arraysum> = DataBuffer::from_buffer_slot(format!("part-{}-{}", array_num-1, my_id)).expect("missing input data.");
    //     buffer.raw_data = reader.raw_data.clone();
    // }
    let mut buffer: DataBuffer<Reader2Arraysum> = DataBuffer::with_slot(format!("part-{}", 0));
    buffer.raw_data = "a".repeat(DATA_SIZE);
    // println!("hhh1");
    let mut count = 0;
    while count < 13 {
        // println!("hhh{}", count);
        let mut buffer2: DataBuffer<Reader2Arraysum> =
            DataBuffer::with_slot(format!("part-{}", count + 1));
        let reader: DataBuffer<Reader2Arraysum> =
            DataBuffer::from_buffer_slot(format!("part-{}", count)).expect("missing input data.");
        buffer2.raw_data = reader.raw_data.clone();
        count += 1;
    }
    // println!("hhh100");
    let reader2: DataBuffer<Reader2Arraysum> =
        DataBuffer::from_buffer_slot(format!("part-{}", 13)).expect("missing input data.");
    let _result = reader2.raw_data.clone();
    // println!("hhh101");
    Ok(().into())
}
