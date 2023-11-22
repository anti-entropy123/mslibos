#![no_std]
use alloc::{collections::BTreeMap, string::String, vec::Vec};
use ms_std::{agent::FaaSFuncResult as Result, fs::File, io::Read, mm::Mmap, println};

extern crate alloc;

#[allow(clippy::result_unit_err)]
#[no_mangle]
pub fn main(_: &BTreeMap<String, String>) -> Result<()> {
    let mut file = File::open("lines.txt").expect("file don't exist?");
    // let mut file_content = Vec::new();
    // file.seek(0);
    // file.read_to_end(&mut file_content).expect("read failed");
    // let file_content = String::from_utf8_lossy(&file_content);
    // println!("file_content: {}", file_content);

    let mmap_area = Mmap::mmap_file(file)?;
    println!("successfully libos mmap");

    let array = mmap_area.as_ref();
    println!("mmap_area content is: {}", array[0]);

    Ok(().into())
}
