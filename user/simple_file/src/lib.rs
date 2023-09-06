#![no_std]

extern crate alloc;

use alloc::{string::String, vec::Vec};
use ms_std::{
    agent::{FaaSFuncResult as Result, Zero},
    fs::File,
    io::{Read, Write},
    println,
};

#[allow(clippy::result_unit_err)]
#[no_mangle]
pub fn main() -> Result<Zero> {
    let path = "lines.txt";

    let data = "Rust LibOS Cool.";
    let mut output = File::create(path)?;
    write!(output, "{}", data).expect("");
    // drop(output);

    let mut input = File::open(path)?;
    let mut buf = Vec::new();
    input.read_to_end(&mut buf).expect("read failed");

    // println!("file: {}", String::from_utf8_lossy(&buf));
    // println!("expect: {}", data);

    assert_eq!(String::from_utf8_lossy(&buf), data);

    Ok(Zero::default().into())
}
