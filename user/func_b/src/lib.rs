#![no_std]

extern crate alloc;

use alloc::string::String;
use ms_std::{
    agent::{DataBuffer, FaaSFuncResult as Result},
    println,
};

#[derive(Default)]
pub struct MyComplexData {
    pub some_int: i64,
    pub some_str: String,
}

#[no_mangle]
#[allow(clippy::result_unit_err)]
pub fn main() -> Result<()> {
    println!("func b");
    let data = DataBuffer::<MyComplexData>::from_buffer();
    if let Some(buffer) = data {
        println!("try recovery data.");
        println!(
            "some_int: {}, some_str={}",
            buffer.some_int, buffer.some_str
        );
    } else {
        println!("buffer is none");
    }

    Ok(().into())
}
