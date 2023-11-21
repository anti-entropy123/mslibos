#![no_std]
use alloc::{collections::BTreeMap, string::String};
use ms_std::{agent::FaaSFuncResult as Result, libos::libos, mm::Mmap, println};

extern crate alloc;

#[allow(clippy::result_unit_err)]
#[no_mangle]
pub fn main(args: &BTreeMap<String, String>) -> Result<()> {
    let mmap_area = Mmap::mmap_file()?;
    println!("successfully libos mmap");

    let array = mmap_area.as_ref();
    println!("mmap_area content is: {}", array[0]);

    Ok(().into())
}
