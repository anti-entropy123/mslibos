#![no_std]
use alloc::{collections::BTreeMap, string::String};
use ms_std::{
    agent::{FaaSFuncResult as Result, Zero},
    libos::MetricEvent::Mem,
    println,
};

extern crate alloc;

#[allow(clippy::result_unit_err)]
#[no_mangle]
pub fn main(args: &BTreeMap<String, String>) -> Result<Zero> {
    println!("Hello, world! id: {}", args["id"]);

    #[cfg(feature = "measure_mem")]
    ms_std::libos::metric(Mem);

    Ok(Zero::default().into())
}
