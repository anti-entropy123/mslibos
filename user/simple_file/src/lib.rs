#![no_std]

extern crate alloc;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use ms_std::{
    agent::FaaSFuncResult as Result,
    fs::File,
    io::{Read, Write},
    println,
};

#[no_mangle]
pub fn main() -> Result<()> {
    let path = "lines.txt";

    /////////////////// test create/write/read. ///////////////////
    let data = "Rust LibOS Cool.";
    let mut output = File::create(path)?;
    write!(output, "{}", data).expect("");
    // drop(output);

    let mut input_file = File::open(path)?;
    let mut file_content_buf = Vec::new();
    input_file
        .read_to_end(&mut file_content_buf)
        .expect("read failed");

    let file_content = String::from_utf8_lossy(&file_content_buf).to_string();
    println!("file_content: {}", file_content);
    // println!("expect: {}", data);

    assert_eq!(file_content, data);

    /////////////////// test seek. ///////////////////
    input_file.seek(0)?;
    file_content_buf.clear();
    input_file
        .read_to_end(&mut file_content_buf)
        .expect("read failed");

    assert_eq!(
        file_content,
        String::from_utf8_lossy(&file_content_buf).to_string()
    );

    /////////////////// test seek. ///////////////////
    assert_eq!(input_file.metadata().unwrap().st_size, file_content.len());

    Ok(().into())
}
