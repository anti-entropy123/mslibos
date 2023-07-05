#![no_std]

extern crate alloc;

use alloc::{borrow::ToOwned, string::String};
use ms_std::{println, DataBuffer};

#[allow(dead_code)]
pub struct MyComplexData {
    pub some_int: i64,
    pub some_str: String,
}

#[allow(clippy::result_unit_err)]
#[no_mangle]
pub fn main() -> Result<DataBuffer, ()> {
    println!("Hello, world!");
    let mut d = DataBuffer::new(MyComplexData {
        some_int: 42,
        some_str: "abc".to_owned(),
    });
    println!("construct d ok.");
    let d = d.to::<MyComplexData>();
    println!("try recovery data.");
    println!("str={}, int={}", d.some_str, d.some_int);
    Ok(DataBuffer::new(()))
}
