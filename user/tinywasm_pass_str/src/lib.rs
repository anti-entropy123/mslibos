#![cfg_attr(feature = "with_libos", no_std)]

use alloc::format;
use ms_std::agent::FaaSFuncResult as Result;
extern crate alloc;
use wasi_api::tinywasm::{Module, ModuleInstance, Store};

#[no_mangle]
pub fn main() -> Result<()> {
    let module = Module::parse_bytes(include_bytes!("../main.wasm"))?;

    let mut store = Store::default();
    let imports = wasi_api::import_all().map_err(|e| format!("import func err: {e}"))?;

    let instance = ModuleInstance::instantiate(&mut store, module, Some(imports))?;
    // assert_eq!(add.call(&mut store, (20))?, 3);
    let _result = instance.start(&mut store);
    // println!("{:?}", unwinding::panic::catch_unwind(|| result));

    Ok(().into())
}
