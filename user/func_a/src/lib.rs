#![no_std]

extern crate alloc;

use alloc::{borrow::ToOwned, string::String};
use ms_std::{
    agent::{DataBuffer, FaaSFuncResult as Result},
    println,
};

#[allow(dead_code)]
#[derive(Default)]
pub struct MyComplexData {
    pub some_int: i64,
    pub some_str: String,
}

#[allow(clippy::result_unit_err)]
#[no_mangle]
pub fn main() -> Result<MyComplexData> {
    println!("Hello, world!");
    let mut d = DataBuffer::<MyComplexData>::default();
    d.some_int = 42;
    d.some_str = "abc".to_owned();

    println!("construct d ok.");
    println!("try recovery data.");
    println!("some_str={}, some_int={}", d.some_str, d.some_int);
    Ok(d)
}
