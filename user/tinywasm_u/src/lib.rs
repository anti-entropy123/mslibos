#![cfg_attr(feature = "with_libos", no_std)]

cfg_if::cfg_if! {
    if #[cfg(feature = "with_libos")] {
        use ms_std::{agent::FaaSFuncResult as Result, println};
        extern crate alloc;
    } else {
        type Result<T> = core::result::Result<T, String>;
        use std::collections::BTreeMap;
    }
}

use wasi_api::tinywasm::{Module, Store, ModuleInstance};

const WASM: &[u8] = include_bytes!("../fibonacci.wasm");

#[no_mangle]
pub fn main() -> Result<()> {
    let module = Module::parse_bytes(WASM)?;
    let mut store = Store::default();
    let instance = ModuleInstance::instantiate(&mut store, module, None)?;
    let fib = instance.exported_func::<i32, i32>(&store, "fibonacci_recursive")?;
    // assert_eq!(add.call(&mut store, (20))?, 3);
    println!("fib(20)={}", fib.call(&mut store, 20)?);

    Ok(().into())
}
