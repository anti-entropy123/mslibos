#![no_std]

extern crate alloc;

use core::mem::size_of;

use alloc::string::String;
use ms_std::{
    agent::{DataBuffer, FaaSFuncResult as Result},
    println,
    time::SystemTime,
};
use ms_std_proc_macro::FaasData;

#[derive(FaasData)]
pub struct MyComplexData {
    pub current_time: SystemTime,
    pub some_int: i64,
    pub some_str: String,
    pub big_data: [u8; 4096 * 7],
}

impl Default for MyComplexData {
    fn default() -> Self {
        Self {
            current_time: SystemTime::now(),
            some_int: Default::default(),
            some_str: Default::default(),
            big_data: [0; 4096 * 7],
        }
    }
}

#[no_mangle]
#[allow(clippy::result_unit_err)]
pub fn main() -> Result<MyComplexData> {
    println!("func b");
    let data = DataBuffer::<MyComplexData>::from_buffer();
    if let Some(buffer) = data {
        let dur = buffer.current_time.elapsed();
        let size = size_of::<MyComplexData>();
        println!(
            "try recovery data. trans data time: {:?}, total_size: {} Bytes, transfer rate: {} MB/s",
            dur,
            size,
            size as u128 / dur.as_micros(),
        );
        // println!("faas args: {:?}", buffer);
        assert_eq!(buffer.some_int, 42);
        Ok(buffer)
    } else {
        println!("buffer is none");
        Err(())
    }
}
