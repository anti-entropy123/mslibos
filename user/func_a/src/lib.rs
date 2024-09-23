#![no_std]

extern crate alloc;

use alloc::{borrow::ToOwned, string::String};
use ms_std::{
    agent::{DataBuffer, FaaSFuncResult as Result},
    time::SystemTime,
};
use ms_std_proc_macro::FaasData;

const DATA_SIZE: usize = 1;

#[allow(dead_code)]
#[derive(FaasData)]
pub struct MyComplexData {
    // pub current_time: SystemTime,
    pub year: i64,
    pub name: String,
    pub big_data: [u8; 4096 * DATA_SIZE],
}

impl Default for MyComplexData {
    fn default() -> Self {
        Self {
            // current_time: SystemTime::now(),
            year: Default::default(),
            name: Default::default(),
            big_data: [0; 4096 * DATA_SIZE],
        }
    }
}

#[allow(clippy::result_unit_err)]
#[no_mangle]
pub fn main() -> Result<MyComplexData> {
    // println!("Hello, world!");
    let mut d = DataBuffer::<MyComplexData>::with_slot("Conference".to_owned());
    d.year = 2025;
    d.name = "Euro".to_owned();

    // for (idx, val) in &mut d.big_data.iter_mut().enumerate() {
    //     *val = (idx % 109usize) as u8
    // }

    // println!("construct d ok.");
    // println!("some_str={}, some_int={}", d.some_str, d.some_int);

    // d.current_time = SystemTime::now();
    Ok(d)
}
