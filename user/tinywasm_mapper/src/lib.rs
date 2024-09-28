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

use alloc::format;
use ms_std::println;
use ms_std::libos::libos;
use ms_hostcall::types::{OpenFlags, OpenMode};

use tinywasm::{Module, Store, ModuleInstance};
use wasi_api::tinywasm;

const WASM: &[u8] = include_bytes!("../mapper.wasm");

#[no_mangle]
pub fn main() -> Result<()> {
    libos!(open("/", OpenFlags::empty(), OpenMode::RD))?;
    // let data_fd = libos!(open("little_fake_data_1.txt", OpenFlags::O_CREAT, OpenMode::RDWR))? as u32;
    // libos!(write(data_fd, b"hello hello hello hello name name name name world world world world"))?;
    // libos!(close(data_fd))?;

    let module = Module::parse_bytes(WASM)?;
    let mut store = Store::default();
    let imports = wasi_api::import_all()?;

    let instance = ModuleInstance::instantiate(&mut store, module, Some(imports))?;
    
    // let mut mem = instance.exported_memory_mut(&mut store, "memory")?;
    // mem.grow(480);
    // drop(mem);    
    
    let main = instance.exported_func::<(), ()>(&store, "_start")?;
    
    if let Err(e) = unwinding::panic::catch_unwind(|| main.call(&mut store, ()).unwrap()) {
        let msg = format!("{:?}", e);
        println!("err msg: {}", msg);
        if msg != "normally exit" {
            // return Err();
        }
    };

    Ok(().into())
}
