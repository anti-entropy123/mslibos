#![no_std]

extern crate alloc;

use ms_std::{
    agent::{DataBuffer, FaaSFuncResult as Result, Zero},
    println,
    time::{SystemTime, UNIX_EPOCH},
};
use ms_std_proc_macro::Verify;

#[allow(dead_code)]
#[derive(Verify, Default)]
pub struct ArrayData {
    // start_times: Vec<u128>,
    // end_times: Vec<u128>,
    n: i32,
}

#[allow(clippy::result_unit_err)]
#[no_mangle]
pub fn main() -> Result<ArrayData> {
    println!("start main()");
    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).as_millis();

    let d: DataBuffer<_> = match DataBuffer::<ArrayData>::from_buffer() {
        Some(array_data) => {
            println!("n: {}", array_data.n);
            array_data
        }
        None => {
            println!("the first function.");
            DataBuffer::default()
        }
    };

    // d.start_times.push(start_time);
    // d.end_times
    //     .push(SystemTime::now().duration_since(UNIX_EPOCH).as_millis());
    // d.n += 1;

    Ok(d)
}
