#![no_std]

extern crate alloc;

use alloc::{string::String, vec::Vec};
use ms_std::{
    agent::FaaSFuncResult as Result,
    fs::File,
    io::{Read, Write},
    println,
};

#[allow(clippy::result_unit_err)]
#[no_mangle]
pub fn main() -> Result<()> {
    let path = "lines.txt";

    let data = "Rust LibOS Cool.";
    let mut output = File::create(path)?;
    write!(output, "{}", data).expect("");
    // drop(output);

    let mut input = File::open(path)?;
    let mut file_content = Vec::new();
    input.read_to_end(&mut file_content).expect("read failed");

    let file_content = String::from_utf8_lossy(&file_content);
    println!("file_content: {}", file_content);
    // println!("expect: {}", data);

    assert_eq!(file_content, data);

    Ok(().into())
}
