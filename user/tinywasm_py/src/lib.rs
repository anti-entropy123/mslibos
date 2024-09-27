#![cfg_attr(feature = "with_libos", no_std)]

cfg_if::cfg_if! {
    if #[cfg(feature = "with_libos")] {
        use ms_std::{agent::FaaSFuncResult as Result};
        extern crate alloc;
        use alloc::{collections::BTreeMap, string::String};
    } else {
        type Result<T> = core::result::Result<T, String>;
        use std::collections::BTreeMap;
    }
}

use ms_std::{
    println,
    time::{SystemTime, UNIX_EPOCH},
};
use tinywasm::{Module, Store};
use wasi_api::tinywasm::{self, ModuleInstance};

const WASM: &[u8] = include_bytes!("../rustpython.wasm");

#[no_mangle]
pub fn main(_args: &BTreeMap<String, String>) -> Result<()> {
    let module = Module::parse_bytes(WASM)?;
    let mut store = Store::default();
    let imports = wasi_api::import_all()?;

    let instance = ModuleInstance::instantiate(&mut store, module, Some(imports))?;
    let main = instance.exported_func::<(), ()>(&store, "_start")?;

    let _start_time = SystemTime::now().duration_since(UNIX_EPOCH).as_millis();

    let result = unwinding::panic::catch_unwind(|| main.call(&mut store, ()).unwrap());
    println!("result: {:?}", result);

    Ok(().into())
}
