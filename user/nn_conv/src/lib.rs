#![no_std]
extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
};

use candle_nn::VarMap;
use ms_std::agent::{DataBuffer, FaaSFuncResult as Result};
use ms_std_proc_macro::FaasData;

#[derive(FaasData)]
struct ConvFaasInput {
    varmap: VarMap,
}

#[warn(clippy::result_unit_err)]
pub fn main(args: &BTreeMap<String, String>) -> Result<()> {
    let mut model_vars: DataBuffer<ConvFaasInput> =
        if let Some(input) = DataBuffer::from_buffer_slot("model_data".to_owned()) {
            input
        } else {
            let mut data = DataBuffer::with_slot("model_data".to_string());
            let m = data.varmap.lock();
            data
        };

    model_vars
        .varmap
        .load(todo!("need complete feature/file-based-mmap"));

    Ok(().into())
}
