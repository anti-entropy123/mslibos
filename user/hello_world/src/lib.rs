#![cfg_attr(feature = "with_libos", no_std)]

cfg_if::cfg_if! {
    if #[cfg(feature = "with_libos")] {
        use ms_std::{agent::FaaSFuncResult as Result, println};
        extern crate alloc;
        use alloc::{collections::BTreeMap, string::String};
    } else {
        type Result<T> = core::result::Result<T, String>;
        use std::collections::BTreeMap;
    }
}

#[no_mangle]
pub fn main() -> Result<()> {
    println!("Hello, world! id: {}" , 1111);
    #[cfg(feature = "measure_mem")]
    {
        use ms_std::libos::MetricEvent::Mem;
        ms_std::libos::metric(Mem);
    }

    Ok(().into())
}
