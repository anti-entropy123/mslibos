#![no_std]
use ms_std::{
    agent::{DataBuffer, FaaSFuncResult as Result},
    println,
};

#[no_mangle]
pub fn main() -> Result<()> {
    let r = DataBuffer::<()>::default();
    println!("Hello, world!");
    Ok(r)
}
