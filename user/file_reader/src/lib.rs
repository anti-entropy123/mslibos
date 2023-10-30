#![no_std]

extern crate alloc;

use alloc::{collections::BTreeMap, format, string::String};
use ms_std::{
    agent::{DataBuffer, FaaSFuncResult as Result},
    fs::File,
    io::Read,
};
use ms_std_proc_macro::FaasData;

#[derive(Default, FaasData)]
struct Reader2Mapper {
    content: String,
}

#[allow(clippy::result_unit_err)]
#[no_mangle]
pub fn main(args: &BTreeMap<String, String>) -> Result<()> {
    let my_id = &args["id"];

    let mut buffer: DataBuffer<Reader2Mapper> = DataBuffer::with_slot(format!("part-{}", my_id));

    buffer.content = {
        let mut f = File::open(&format!("fake_data_{}.txt", my_id))?;
        let mut buf = String::new();
        f.read_to_string(&mut buf).expect("read file failed.");
        buf
    };

    Ok(().into())
}
