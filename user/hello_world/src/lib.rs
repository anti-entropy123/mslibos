#![no_std]
use ms_std::{
    agent::{FaaSFuncResult as Result, Zero},
    libos::MetricEvent::Mem,
    println,
};

#[allow(clippy::result_unit_err)]
#[no_mangle]
pub fn main() -> Result<Zero> {
    println!("Hello, world!");

    #[cfg(feature = "measure_mem")]
    ms_std::libos::metric(Mem);

    Ok(Zero::default().into())
}
