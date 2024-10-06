#![no_std]

use alloc::{string::ToString, vec::Vec};
use ms_std::{agent::FaaSFuncResult as Result, args, println, libos::libos};
use wasmtime_wasi_api::wasmtime;
use wasmtime::{Engine, Instance, Linker, Module, Store};

// use wasmtime::{AsContext, Caller};
extern crate alloc;

static CWASM: &[u8] = include_bytes!("../write.cwasm");

#[repr(C)]
struct WasiCiovec {
    buf: u32,
    buf_len: u32,
}

#[no_mangle]
pub fn main() -> Result<()> {
    let id = args::get("id").unwrap();
    println!("rust: wasmtime hello, world! id: {}", id);

    let engine = Engine::default();
    let module = unsafe { Module::deserialize(&engine, CWASM) }.map_err(|e| e.to_string())?;

    let mut linker = Linker::new(&engine);

    // let fd_write = |mut caller: Caller<'_, ()>, fd: i32, iovs_ptr: i32, iovs_len: i32, retptr: i32| -> i32 {
    //     #[cfg(feature = "log")]
    //     {
    //         println!("[Debug] Invoke into fd_write");
    //         println!(
    //             "args: fd: {:?}, iovs_ptr: {:?}, iovs_len: {:?}, retptr: {:?}",
    //             fd, iovs_ptr, iovs_len, retptr
    //         );
    //     }
        
    //     let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
    //     let mut write_size: usize = 0;

    //     for i in 0..iovs_len {
    //         let offset: usize = iovs_ptr as usize + i as usize * core::mem::size_of::<WasiCiovec>();
    //         let mut iovs = [0; 8];
    //         memory.read(&caller, offset, &mut iovs);
    //         let iovs: &WasiCiovec = unsafe { &*(iovs.as_ptr() as *const WasiCiovec) };
    //         let mut buf: Vec<u8> = Vec::with_capacity(iovs.buf_len as usize);
    //         buf.resize(iovs.buf_len as usize, 0);
    //         memory.read(&caller, iovs.buf as usize, &mut buf);
    //         write_size += libos!(write(fd as u32, buf.as_slice())).unwrap();
    //     }

    //     #[cfg(feature = "log")]
    //     println!("write_size: {:?}", write_size);
    //     memory.write(&mut caller, retptr as usize, &write_size.to_ne_bytes());
    //     0
    // };

    // let proc_exit = |mut caller: Caller<'_, ()>, code: i32| {
    //     #[cfg(feature = "log")]
    //     {
    //         println!("[Debug] Invoke into proc_exit");
    //         println!("args: code: {:?}", code);
    //     }
    //     panic!("normally exit")
    // };

    // linker
    //     .func_wrap(
    //         "wasi_snapshot_preview1",
    //         "fd_write",
    //         fd_write,
    //     )
    //     .unwrap();
    // linker
    //     .func_wrap(
    //         "wasi_snapshot_preview1",
    //         "proc_exit",
    //         proc_exit,
    //     )
    //     .unwrap();

    wasmtime_wasi_api::import_all(&mut linker);

    let mut store = Store::new(&engine, ());

    let instance = linker.instantiate(&mut store, &module)?;

    let main = instance
        .get_typed_func::<(), ()>(&mut store, "_start")
        .map_err(|e| e.to_string())?;

    main.call(store, ()).map_err(|e| e.to_string())?;

    Ok(().into())
}
