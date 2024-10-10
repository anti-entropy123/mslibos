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

use alloc::{string::ToString, vec::Vec};
use ms_std::{
    libos::libos,
    println,
    time::{SystemTime, UNIX_EPOCH},
};
use tinywasm::{Module, Store};
use wasi_api::tinywasm::{self, ModuleInstance};
use ms_hostcall::types::{OpenFlags, OpenMode};

const WASM: &[u8] = include_bytes!("../python.wasm");

#[no_mangle]
pub fn main(_args: &BTreeMap<String, String>) -> Result<()> {
    libos!(open("/", OpenFlags::empty(), OpenMode::RD))?;
    let mut wasi_args: Vec<String> = Vec::new();
    wasi_args.push("python.wasm".to_string());
    wasi_args.push("/wasm_bench/pass_str.py".to_string());

    let module = Module::parse_bytes(WASM)?;
    let mut store = Store::default();
    let imports = wasi_api::import_all()?;

    wasi_api::set_wasi_args(store.id(), wasi_args);

    let instance = ModuleInstance::instantiate(&mut store, module, Some(imports))?;

    let _result = unwinding::panic::catch_unwind(|| instance.start(&mut store));

    // println!("before _start");

    // let main = instance.exported_func::<(), ()>(&store, "_start")?;

    // let _result = unwinding::panic::catch_unwind(|| main.call(&mut store, ()).unwrap());
    println!("pass.rs: result: {:?}", _result);

    core::mem::forget(store);
    Ok(().into())
}
