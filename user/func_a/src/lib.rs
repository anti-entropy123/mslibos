#![no_std]

extern crate alloc;

use alloc::{borrow::ToOwned, string::String};
use ms_std::{
    agent::{DataBuffer, FaaSFuncResult as Result},
    time::SystemTime,
};
use ms_std_proc_macro::FaasData;

#[allow(dead_code)]
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

#[allow(clippy::result_unit_err)]
#[no_mangle]
pub fn main() -> Result<MyComplexData> {
    // println!("Hello, world!");
    let mut d = DataBuffer::<MyComplexData>::default();
    d.some_int = 42;
    d.some_str = "abc".to_owned();

    // println!("construct d ok.");
    // println!("some_str={}, some_int={}", d.some_str, d.some_int);

    d.current_time = SystemTime::now();
    Ok(d)
}
