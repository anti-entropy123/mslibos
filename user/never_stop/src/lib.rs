#![cfg_attr(feature = "with_libos", no_std)]

use core::time::Duration;

use ms_std::time::sleep;

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
    sleep(Duration::from_secs(100000));

    Ok(().into())
}
