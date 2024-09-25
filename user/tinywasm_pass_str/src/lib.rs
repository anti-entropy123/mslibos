#![cfg_attr(feature = "with_libos", no_std)]

use alloc::format;
use ms_std::{
    agent::{FaaSFuncError, FaaSFuncResult as Result},
    println,
};
extern crate alloc;
use wasi_api::tinywasm::{Module, Store};

const WASM: &[u8] = include_bytes!("../main.wasm");

#[no_mangle]
pub fn main() -> Result<()> {
    let module = Module::parse_bytes(WASM)?;

    let mut store = Store::default();
    let imports = wasi_api::import_all().map_err(|e| format!("import func err: {e}"))?;

    let instance = module.instantiate(&mut store, Some(imports))?;
    // assert_eq!(add.call(&mut store, (20))?, 3);
    println!(
        "{:?}",
        unwinding::panic::catch_unwind(|| instance.start(&mut store))
    );

    Ok(().into())
}
