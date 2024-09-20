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

use tinywasm::{FuncContext, Imports, Module, Store};

const WASM: &[u8] = include_bytes!("../write.wasm");

#[repr(C)]
struct WasiCiovec {
    buf: *const u8,
    buf_len: u32,
}

#[no_mangle]
pub fn main(args: &BTreeMap<String, String>) -> Result<()> {
    let module = Module::parse_bytes(WASM)?;
    let mut store = Store::default();
    let mut imports = Imports::new();

    imports
        .define("wasi_snapshot_preview1", "fd_write", Extern::typed_func(|ctx: FuncContext<'_>, args: (i32, i32, i32, i32)| -> Result<i32> {
            let fd = args.0 as u32;
            let ptr = args.1 as u32;
            let iovs_len = args.2 as u32;
            let mem = ctx.exported_memory("memory")?;

            for i in (0..iovs_len) {
                let addr = mem.load(ptr + i * std::mem::size_of::<u32>(), std::mem::size_of::<u32>())? as u32;
                let wasi_ciovec = mem.load(addr, std::mem::size_of::<WasiCiovec>())? as WasiCiovec;
                let buf = mem.load(wasi_ciovec.buf, wasi_ciovec.buf_len)? as &[u8];
                libos!(write(1, buf));
            }
            
            Ok((0))
        }))?
        .define("wasi_snapshot_preview1", "proc_exit", Extern::typed_func(|_: FuncContext<'_>, ()| -> Result<i32> {
            Ok((0))
        }))?;
    let instance = module.instantiate(&mut store, Some(imports))?;
    let main = instance.exported_func::<(), ())>(&store, "_start")?;
    // assert_eq!(add.call(&mut store, (20))?, 3);
    // println!("fib(20)={}", fib.call(&mut store, 20)?);
    main.call(&mut store);

    Ok(().into())
}
