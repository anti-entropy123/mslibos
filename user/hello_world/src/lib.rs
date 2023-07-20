#![no_std]
use ms_std::{agent::FaaSFuncResult as Result, println};

#[allow(clippy::result_unit_err)]
#[no_mangle]
pub fn main() -> Result<()> {
    // let r = DataBuffer::<()>::default();
    println!("Hello, world!");
    Ok(().into())
}
