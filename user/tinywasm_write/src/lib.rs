#![cfg_attr(feature = "with_libos", no_std)]

cfg_if::cfg_if! {
    if #[cfg(feature = "with_libos")] {
        use ms_std::{agent::FaaSFuncResult as Result};
        extern crate alloc;
    } else {
        type Result<T> = core::result::Result<T, String>;
        use std::collections::BTreeMap;
    }
}

use ms_std::{agent::FaaSFuncError, println};

use tinywasm::{Module, Store};
use wasi_api::tinywasm::{self, ModuleInstance};

const WASM: &[u8] = include_bytes!("../write.wasm");

pub fn run_wasm_app(wasm: &[u8]) -> core::result::Result<(), FaaSFuncError> {
    let module = Module::parse_bytes(wasm)?;
    let mut store = Store::default();
    let imports = wasi_api::import_all()?;

    let instance = ModuleInstance::instantiate(&mut store, module, Some(imports))?;
    let main = instance.exported_func::<(), ()>(&store, "_start")?;
    main.call(&mut store, ())?;

    Ok(())
}

#[no_mangle]
pub fn main() -> Result<()> {
    // assert_eq!(add.call(&mut store, (20))?, 3);
    // println!("fib(20)={}", fib.call(&mut store, 20)?);
    let result = unwinding::panic::catch_unwind(|| run_wasm_app(WASM));
    // println!("result = {:#?}", result);
    if let Ok(val) = result {
        val?
    } else {
        println!("run wasm ok.")
    }

    // if let Err(e) =  {
    //     let msg = format!("{:?}", e);
    //     println!("err msg: {}", msg);
    //     if msg != "normally exit" {
    //         // return Err();
    //     }
    // };

    Ok(().into())
}
