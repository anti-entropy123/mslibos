#![no_std]
use alloc::{collections::BTreeMap, string::String};
use ms_std::{agent::FaaSFuncResult as Result, libos::libos, mm::Mmap, println};

extern crate alloc;

#[allow(clippy::result_unit_err)]
#[no_mangle]
pub fn main(args: &BTreeMap<String, String>) -> Result<()> {
    let MmapArea = Mmap::mmap_file();
    println!("successfully libos mmap");

    Ok(().into())
}
