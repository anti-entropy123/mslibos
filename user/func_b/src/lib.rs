#![no_std]

extern crate alloc;

use alloc::string::String;
use ms_std::{agent::DataBuffer, println};
use ms_std_proc_macro::Verify;

#[derive(Default, Verify, Debug)]
pub struct MyComplexData {
    pub some_int: i64,
    // pub some_str: String,
}

#[no_mangle]
#[allow(clippy::result_unit_err)]
pub fn main() -> Result<(), ()> {
    println!("func b");
    let data = DataBuffer::<MyComplexData>::from_buffer();
    if let Some(buffer) = data {
        println!("try recovery data.");
        println!("faas args: {:?}", buffer);
    } else {
        println!("buffer is none");
    }

    Ok(())
}
