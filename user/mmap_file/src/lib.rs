#![no_std]
use alloc::{collections::BTreeMap, string::String};
use ms_std::{agent::FaaSFuncResult as Result, fs::File, mm::Mmap, println};

extern crate alloc;

#[allow(clippy::result_unit_err)]
#[no_mangle]
pub fn main(_: &BTreeMap<String, String>) -> Result<()> {
    let file = File::open("lines.txt").expect("file don't exist?");

    let mmap_area = Mmap::mmap_file(file)?;
    // println!("successfully libos mmap");

    let array = mmap_area.as_ref();
    println!("mmap_area content is: {:?}", String::from_utf8_lossy(array));

    Ok(().into())
}
