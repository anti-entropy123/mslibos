#![no_std]
use ms_std::{
    agent::{FaaSFuncResult as Result, Zero},
    println,
};

#[allow(clippy::result_unit_err)]
#[no_mangle]
pub fn main() -> Result<Zero> {
    // let r = DataBuffer::<()>::default();
    println!("Hello, world!");
    Ok(Zero::default().into())
}
