#![cfg_attr(feature = "with_libos", no_std)]

cfg_if::cfg_if! {
    if #[cfg(feature = "with_libos")] {
        use ms_std::{agent::FaaSFuncResult as Result, println, libos::libos, fs::File};
        extern crate alloc;
        use alloc::{collections::BTreeMap, string::String};
    } else {
        type Result<T> = core::result::Result<T, String>;
        use std::collections::BTreeMap;
    }
}

#[no_mangle]
pub fn main(args: &BTreeMap<String, String>) -> Result<()> {
    println!("Hello, world! id: {}", args["id"]);

    libos!(addrinfo("localhost"));
    if let Err(e) = File::open(".") {
        println!("{:#?}", e);
    }

    Ok(().into())
}
