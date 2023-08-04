#![no_std]

extern crate alloc;

use alloc::string::String;
use ms_std::{
    agent::{DataBuffer, FaaSFuncResult as Result},
    println,
};
use ms_std_proc_macro::Verify;

#[derive(Verify, Debug)]
pub struct MyComplexData {
    pub some_int: i64,
    pub some_str: String,
    pub big_data: [u8; 4096],
}

impl Default for MyComplexData {
    fn default() -> Self {
        Self {
            some_int: Default::default(),
            some_str: Default::default(),
            big_data: [0; 4096],
        }
    }
}

#[no_mangle]
#[allow(clippy::result_unit_err)]
pub fn main() -> Result<MyComplexData> {
    println!("func b");
    let data = DataBuffer::<MyComplexData>::from_buffer();
    if let Some(buffer) = data {
        println!("try recovery data.");
        // println!("faas args: {:?}", buffer);
        Ok(buffer)
    } else {
        println!("buffer is none");
        Err(())
    }
}
