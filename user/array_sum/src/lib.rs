#![no_std]

extern crate alloc;

// use alloc::vec::Vec;
use alloc::{borrow::ToOwned, format, string::String, vec::Vec};
// use ms_std::{
//     agent::{DataBuffer, FaaSFuncResult as Result},
//     println,
//     time::{SystemTime, UNIX_EPOCH},
// };
use ms_std::{
    agent::{DataBuffer, FaaSFuncResult as Result},
    args, println,
};
use ms_std_proc_macro::FaasData;

#[derive(Default, FaasData)]
struct Reader2Arraysum {
    raw_data: String,
}

// #[allow(dead_code)]
// #[derive(FaasData, Default, Clone)]
// pub struct ArrayData {
//     // start_times: Vec<u128>,
//     // end_times: Vec<u128>,
//     n: i32,
// }

#[allow(clippy::result_unit_err)]
#[no_mangle]
pub fn main() -> Result<()> {
// pub fn main() -> Result<ArrayData> {/
    // println!("start main()");
    // let start_time = SystemTime::now().duration_since(UNIX_EPOCH).as_millis();

    // let mut d: ArrayData = match DataBuffer::<ArrayData>::from_buffer() {
    //     Some(array_data) => {
    //         println!("n: {}", array_data.n);
    //         array_data.clone()
    //     }
    //     None => {
    //         println!("the first function.");
    //         Default::default()
    //     }
    // };

    // d.start_times.push(start_time);
    // d.end_times
    //     .push(SystemTime::now().duration_since(UNIX_EPOCH).as_millis());
    // d.n += 1;

    // if d.n == 10 {
    //     println!(
    //         "start_time: {:?}, end_time: {:?}",
    //         d.start_times, d.end_times
    //     );
    // };

    // let mut buffer = DataBuffer::default();
    // *buffer = d;

    let my_id = args::get("id").unwrap();
    let array_num: u64 = args::get("array_num")
        .expect("missing arg reducer_num")
        .parse()
        .unwrap_or_else(|_| panic!("bad arg, array_num={}", args::get("array_num").unwrap()));
    // println!("get id {}", my_id);
    let reader: DataBuffer<Reader2Arraysum> =
        DataBuffer::from_buffer_slot(format!("part-{}-{}", array_num-1, my_id)).expect("missing input data.");
    // println!("read buffer");
    let mut buffer: DataBuffer<Reader2Arraysum> =
        DataBuffer::with_slot(format!("part-{}-{}", array_num, my_id));
    buffer.raw_data = reader.raw_data.clone();
    Ok(().into())
}
