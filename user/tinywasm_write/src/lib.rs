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

use alloc::format;
use ms_std::println;
use tinywasm::Extern;
use tinywasm::{FuncContext, Imports, Module, Store};

use ms_std::libos::libos;

const WASM: &[u8] = include_bytes!("../write.wasm");

#[repr(C)]
struct WasiCiovec {
    buf: u32,
    buf_len: u32,
}

#[no_mangle]
pub fn main(_args: &BTreeMap<String, String>) -> Result<()> {
    let module = Module::parse_bytes(WASM)?;
    let mut store = Store::default();
    let mut imports = Imports::new();

    let fd_write = Extern::typed_func(
        |mut ctx: FuncContext<'_>, args: (i32, i32, i32, i32)| -> tinywasm::Result<i32> {
            let fd = args.0 as u32;
            let ptr = args.1 as usize;
            let iovs_len = args.2 as usize;
            let mem = ctx.exported_memory("memory")?;

            for i in 0..iovs_len {
                let offset = ptr + i * core::mem::size_of::<WasiCiovec>();
                let addr: &[u8] = mem.load(offset, core::mem::size_of::<WasiCiovec>())?;
                let addr: &WasiCiovec = unsafe { &*(addr.as_ptr() as *const WasiCiovec) };
                // let wasi_ciovec = mem.load(addr, std::mem::size_of::<WasiCiovec>())? as WasiCiovec;
                let buf = mem.load(addr.buf as usize, addr.buf_len as usize)? as &[u8];
                libos!(write(fd, buf)).unwrap();
            }

            Ok(0)
        },
    );

    let proc_exit = Extern::typed_func(|_: FuncContext<'_>, _args: i32| -> tinywasm::Result<()> {
        panic!("normally exit");
        Ok(())
    });

    // let fd_write = Extern::typed_func(
    //     |ctx: FuncContext<'_>, args: (i32, i32, i32, i32)| -> tinywasm::Result<i32> { Ok(0) },
    // );

    imports
        .define("wasi_snapshot_preview1", "fd_write", fd_write)?
        .define("wasi_snapshot_preview1", "proc_exit", proc_exit)?;

    let instance = module.instantiate(&mut store, Some(imports))?;
    let main = instance.exported_func::<(), ()>(&store, "_start")?;
    // assert_eq!(add.call(&mut store, (20))?, 3);
    // println!("fib(20)={}", fib.call(&mut store, 20)?);

    if let Err(e) = unwinding::panic::catch_unwind(|| main.call(&mut store, ()).unwrap()) {
        let msg = format!("{:?}", e);
        println!("err msg: {}", msg);
        if msg != "normally exit" {
            // return Err();
        }
    };

    Ok(().into())
}
