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

use alloc::{format, vec};
use alloc::vec::Vec;
use ms_std::{
    libos::{libos},
    println,
    time::{SystemTime, UNIX_EPOCH},
};
use wasi_api::tinywasm;
use tinywasm::Extern;
use tinywasm::{FuncContext, Imports, Module, Store};

// use ms_std::libos::libos;

#[repr(C)]
struct WasiCiovec {
    buf: u32,
    buf_len: u32,
}

const WASM: &[u8] = include_bytes!("../rustpython.wasm");

#[no_mangle]
pub fn main(_args: &BTreeMap<String, String>) -> Result<()> {
    let module = Module::parse_bytes(WASM)?;
    let mut store = Store::default();
    let imports = wasi_api::import_all()?;

    let instance = module.instantiate(&mut store, Some(imports))?;
    let main = instance.exported_func::<(), ()>(&store, "_start")?;
    
    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).as_millis();

    if let Err(e) = unwinding::panic::catch_unwind(|| main.call(&mut store, ()).unwrap()) {
        let msg = format!("{:?}", e);
        println!("err msg: {:?}", msg);
        if msg != "normally exit" {
            // return Err();
        }
    };

    Ok(().into())
}
