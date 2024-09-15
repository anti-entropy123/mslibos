#![no_std]

use alloc::{borrow::ToOwned, collections::BTreeMap, string::String};
use ms_std::{fs::File, io::Read, prelude::*};
use ms_std_proc_macro::FaasData;

#[derive(Default, FaasData)]
struct VecArg {
    content: String,
}

#[no_mangle]
pub fn main(args: &BTreeMap<String, String>) -> Result<()> {
    let slot_name = &args["slot_name"];
    let input_file = &args["input_file"];

    let mut buffer: DataBuffer<VecArg> = DataBuffer::with_slot(slot_name.to_owned());

    buffer.content = {
        let mut f = File::open(input_file)?;
        let mut buf = String::new();
        f.read_to_string(&mut buf).expect("read file failed.");
        buf
    };

    Ok(().into())
}
