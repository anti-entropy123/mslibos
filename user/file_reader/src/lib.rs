#![no_std]

use alloc::string::String;
use ms_std::{
    args,
    fs::File,
    io::Read,
    prelude::*,
    println,
    time::{SystemTime, UNIX_EPOCH},
};
// use ms_std_proc_macro::FaasData;

// #[derive(Default, FaasData)]
// struct VecArg {
//     content: String,
// }

#[no_mangle]
pub fn main() -> Result<()> {
    // let slot_name = args::get("slot_name").unwrap();
    let input_file = args::get("input_file").unwrap();

    // let mut buffer: DataBuffer<VecArg> = DataBuffer::with_slot(slot_name.to_owned());
    let mut f = File::open(input_file)?;
    let mut buf = String::new();
    println!(
        "read_start: {}",
        SystemTime::now().duration_since(UNIX_EPOCH).as_micros() as f64 / 1000000f64
    );
    // buffer.content = {
    //     let mut f = File::open(input_file)?;
    //     let mut buf = String::new();
    //     f.read_to_string(&mut buf).expect("read file failed.");
    //     buf
    // };
    // println!("buffer_long={}", buffer.content.len());
    f.read_to_string(&mut buf).expect("read file failed.");
    println!(
        "read_end: {}",
        SystemTime::now().duration_since(UNIX_EPOCH).as_micros() as f64 / 1000000f64
    );

    Ok(().into())
}
