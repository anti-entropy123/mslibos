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

use tinywasm::{Module, Store};

const WASM: &[u8] = include_bytes!("../fibonacci.wasm");

#[no_mangle]
pub fn main(args: &BTreeMap<String, String>) -> Result<()> {
    let module = Module::parse_bytes(WASM)?;
    let mut store = Store::default();
    let instance = module.instantiate(&mut store, None)?;
    let fib = instance.exported_func::<i32, i32>(&store, "fibonacci_recursive")?;
    // assert_eq!(add.call(&mut store, (20))?, 3);
    println!("fib(20)={}", fib.call(&mut store, 20)?);

    Ok(().into())
}
