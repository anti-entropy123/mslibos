#![no_std]

extern crate alloc;

use core::{mem::size_of, ptr};

use alloc::{borrow::ToOwned, string::String};
use ms_std::{
    agent::{DataBuffer, FaaSFuncResult as Result},
    println,
    time::SystemTime,
};
use ms_std_proc_macro::FaasData;

const DATA_SIZE: usize = 10000;

#[derive(FaasData)]
pub struct MyComplexData {
    pub current_time: SystemTime,
    pub year: i64,
    pub name: String,
    pub big_data: [u8; 4096 * DATA_SIZE],
}

impl Default for MyComplexData {
    fn default() -> Self {
        Self {
            current_time: SystemTime::now(),
            year: Default::default(),
            name: Default::default(),
            big_data: [0; 4096 * DATA_SIZE],
        }
    }
}

#[no_mangle]
#[allow(clippy::result_unit_err)]
pub fn main() -> Result<MyComplexData> {
    println!("func b");
    let data = DataBuffer::<MyComplexData>::from_buffer_slot("Conference".to_owned());
    if let Some(buffer) = data {
        let dur = buffer.current_time.elapsed();
        let size = size_of::<MyComplexData>();
        for i in 0..buffer.big_data.len() {
            let _ = unsafe { ptr::read_volatile((&buffer.big_data[i]) as *const u8) };
        }
        println!(
            "try recovery data. trans data time: {:?}, total_size: {} Bytes, transfer rate: {} MB/s",
            dur,
            size,
            size as u128 / dur.as_micros(),
        );
        println!("{}Sys, {}", buffer.name, buffer.year);
        assert_eq!(buffer.year, 2025);
        Ok(buffer)
    } else {
        Err("buffer is none")?
    }
}
