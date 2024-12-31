#![no_std]

extern crate alloc;

use alloc::borrow::ToOwned;
use ms_std::{
    agent::{DataBuffer, FaaSFuncResult as Result},
    println,
    time::SystemTime,
};
use ms_std_proc_macro::FaasData;

const DATA_SIZE: usize = 1024 * 1024 * 16 / 8;

#[derive(FaasData)]
pub struct MyComplexData {
    data: [u64; DATA_SIZE],
}

impl Default for MyComplexData {
    fn default() -> Self {
        Self {
            data: [0; DATA_SIZE],
        }
    }
}

#[no_mangle]
#[allow(clippy::result_unit_err)]
pub fn main() -> Result<MyComplexData> {
    println!("func b");
    let func_b_start = SystemTime::now();
    let data = DataBuffer::<MyComplexData>::from_buffer_slot("Conference".to_owned());
    if let Some(buffer) = data {
        for i in 0..buffer.data.len() {
            let _ = unsafe { core::ptr::read_volatile((&buffer.data[i]) as *const u64) };
        }
        println!(
            "phase34_dur={}",
            SystemTime::now().duration_since(func_b_start).as_nanos()
        );
        Ok(buffer)
    } else {
        Err("buffer is none")?
    }
}
