#![no_std]
use alloc::{collections::BTreeMap, string::String};
use ms_std::{agent::FaaSFuncResult as Result, fs::File, mm::Mmap, println};

extern crate alloc;

#[allow(clippy::result_unit_err)]
#[no_mangle]
pub fn main(_: &BTreeMap<String, String>) -> Result<()> {
    let file1 = File::open("lines.txt").expect("file1 don't exist?");
    let file2 = File::open("fake_data_0.txt").expect("file2 don't exist?");

    let mmap_area1 = Mmap::mmap_file(file1)?;
    let mmap_area2 = Mmap::mmap_file(file2)?;
    // println!("successfully libos mmap");

    println!(
        "mmap_area2 content is: {}",
        String::from_utf8_lossy(mmap_area2.as_ref())
    );
    println!(
        "mmap_area1 content is: {}",
        String::from_utf8_lossy(mmap_area1.as_ref())
    );

    Ok(().into())
}
