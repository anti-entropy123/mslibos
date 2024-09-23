#![no_std]

use alloc::{borrow::ToOwned, string::String};
use ms_std::{args, fs::File, io::Read, prelude::*};
use ms_std_proc_macro::FaasData;

#[derive(Default, FaasData)]
struct VecArg {
    content: String,
}

#[no_mangle]
pub fn main() -> Result<()> {
    let slot_name = args::get("slot_name").unwrap();
    let input_file = args::get("input_file").unwrap();
    println!(
        "file_reader: slot_name={}, input_file={}",
        slot_name, input_file
    );

    let mut buffer: DataBuffer<VecArg> = DataBuffer::with_slot(slot_name.to_owned());

    buffer.content = {
        let mut f = File::open(input_file)?;
        let mut buf = String::new();
        f.read_to_string(&mut buf).expect("read file failed.");
        buf
    };

    Ok(().into())
}
