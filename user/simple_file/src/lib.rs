#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use ms_std::{
    agent::{FaaSFuncResult as Result, Zero},
    fs::File,
    io::{Read, Write},
};

#[allow(clippy::result_unit_err)]
#[no_mangle]
pub fn main() -> Result<Zero> {
    let path = "lines.txt";

    let mut output = File::create(path)?;
    write!(output, "Rust\n💖\nFun").expect("");

    let mut input = File::open(path)?;
    let mut buf = Vec::new();
    input.read_to_end(&mut buf).expect("read failed");

    Ok(Zero::default().into())
}
