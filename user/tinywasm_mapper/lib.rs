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

use core::slice;

use alloc::format;
use ms_std::println;
use tinywasm::Extern;
use tinywasm::{FuncContext, Imports, Module, Store};

use ms_std::libos::libos;

const WASM: &[u8] = include_bytes!("../mapper.wasm");

#[repr(C)]
struct WasiCiovec {
    buf: u32,
    buf_len: u32,
}

#[repr(C)]
struct WasiFdstat {
    filetype: u8,
    flags: u16,
    rights: u64,
    rights_inher: u64,
}

#[repr(C)]
struct WasiPrestatDir {
    dirname_len: u64,
}

#[repr(C)]
struct WasiPrestatUt {
    dir: WasiPrestatDir,
}

#[repr(C)]
struct WasiPrestatT {
    tag: u8,
    u: WasiPrestatUt,
}

#[no_mangle]
pub fn main(_args: &BTreeMap<String, String>) -> Result<()> {
    let module = Module::parse_bytes(WASM)?;
    let mut store = Store::default();
    let mut imports = Imports::new();

    let fd_read = Extern::typed_func(
        |mut ctx: FuncContext<'_>, args: (i32, i32, i32, i32)| -> tinywasm::Result<i32> {
            let fd = args.0 as u32;
            let ptr = args.1 as usize;
            let iovs_len = args.2 as usize;
            let mut mem = ctx.exported_memory_mut("memory")?;
            let mut read_size: usize = 0;

            for i in 0..iovs_len {
                let offset = ptr + i * core::mem::size_of::<u32>();
                let addr: &[u8] = mem.load(offset, core::mem::size_of::<u32>())?;
                let addr: &WasiCiovec = unsafe { &*(addr.as_ptr() as *const WasiCiovec) };
                let buf: &[u8] = mem.load(addr.buf as usize, addr.buf_len as usize)?;
                let buf: &mut [u8] = unsafe { slice::from_raw_parts_mut(buf.as_ptr() as usize as *mut u8, addr.buf_len as usize) };
                read_size += libos!(read(fd, buf)).unwrap();
            }

            mem.store(args.3 as usize, core::mem::size_of::<u32>(), &read_size.to_ne_bytes() )?;

            Ok(0)
        },
    );

    let fd_seek = Extern::typed_func(
        |mut ctx: FuncContext<'_>, args: (i32, i64, i32, i32)| -> tinywasm::Result<i32> {
            let fd = args.0 as u32;
            let offset = args.1;
            let whence = args.2;
            let pos = offset as u32;

            libos!(lseek(fd, pos)).unwrap();
            
            Ok(0)
        },
    );

    let fd_write = Extern::typed_func(
        |mut ctx: FuncContext<'_>, args: (i32, i32, i32, i32)| -> tinywasm::Result<i32> {
            let fd = args.0 as u32;
            let ptr = args.1 as usize;
            let iovs_len = args.2 as usize;
            let mem = ctx.exported_memory("memory")?;

            for i in 0..iovs_len {
                let offset = ptr + i * core::mem::size_of::<u32>();
                let addr: &[u8] = mem.load(offset, core::mem::size_of::<u32>())?;
                let addr: &WasiCiovec = unsafe { &*(addr.as_ptr() as *const WasiCiovec) };
                let buf = mem.load(addr.buf as usize, addr.buf_len as usize)? as &[u8];
                libos!(write(fd, buf)).unwrap();
            }

            Ok(0)
        },
    );

    let fd_close= Extern::typed_func(|_: FuncContext<'_>, _args: i32| -> tinywasm::Result<i32> {
        let fd = _args as u32;
        libos!(close(fd)).unwrap();

        Ok(0)
    });

    let proc_exit = Extern::typed_func(|_: FuncContext<'_>, _args: i32| -> tinywasm::Result<()> {
        panic!("normally exit");
        Ok(())
    });

    let fd_fdstat_get = Extern::typed_func(
        |mut ctx: FuncContext<'_>, args: (i32, i32)| -> tinywasm::Result<i32> {
            let fd = args.0 as u32;
            let addr = args.1;
            // let addr: &WasiFdstat = unsafe { &*(addr.as_ptr() as *const WasiFdstat) };
            
            let fdstat = WasiFdstat {
                fs_filetype: 4, 
                fs_flags: 0,    
                fs_rights_base: 0xFFFFFFFFFFFFFFFF, 
                fs_rights_inheriting: 0xFFFFFFFFFFFFFFFF, 
            };

            // 计算 WasiFdstat 结构体的大小
            let size = core::mem::size_of::<WasiFdstat>();

            // 使用 mem_store 将数据写入 WASM 内存
            mem.store(addr, &fdstat, size)?;

            Ok(0)
        },
    );

    let fd_fdstat_set_flags = Extern::typed_func(
        | mut ctx: FuncContext<'_>, args: (i32, i32)| -> tinywasm::Result<i32> {
            let fd = args.0 as u32;
            let flag = args.1 as u16;

            Ok(0)
        }
    )

    let fd_prestat_get = Extern::typed_func(
        | mut ctx: FuncContext<'_>, args: (i32, i32)| -> tinywasm::Result<i32> {
            // let fd = args.0 as u32;
            // let addr = args.1;
            
            // let prestat = WasiPrestatT {
            //     tag: 0, // 假设 tag 为 0，表示这是一个目录
            //     u: WasiPrestatUt {
            //         dir: WasiPrestatDir {
            //             pr_name_len: 12, // 示例长度
            //         },
            //     },
            // };

            // // addr 是指向 WASM 内存的指针
            // let size = core::mem::size_of::<WasiPrestatT>();

            // // 使用 mem_store 将数据写入 WASM 内存
            // mem.store(addr, &prestat, size)?;

            // Ok(0)

            return Ok(54); // 返回未打开目录的错误__WASI_ERRNO_NOTDIR，提示调用方自己打开
        }
    )

    let fd_prestat_dir_name = Extern::typed_func(
        | mut ctx: FuncContext<'_>, args: (i32, i32, i32)| -> tinywasm::Result<i32> {
            let fd = args.0 as u32;
            let addr = args.1;
            let path_len = args.2 as u32;

            return Ok(54); // 返回未打开目录的错误__WASI_ERRNO_NOTDIR，提示调用方自己打开
        }
    )

    imports
        .define("wasi_snapshot_preview1", "proc_exit", proc_exit)?
        .define("wasi_snapshot_preview1", "fd_write", fd_write)?
        .define("wasi_snapshot_preview1", "fd_read", fd_read)?
        .define("wasi_snapshot_preview1", "fd_close", fd_close)?
        .define("wasi_snapshot_preview1", "fd_seek", fd_seek)?;
        .define("wasi_snapshot_preview1", "fd_fdstat_get", fd_fdstat_get)?;
        .define("wasi_snapshot_preview1", "fd_fdstat_set_flags", fd_fdstat_set_flags)?;
        .define("wasi_snapshot_preview1", "fd_prestat_get", fd_prestat_get)?;
        .define("wasi_snapshot_preview1", "fd_prestat_dir_name", fd_prestat_dir_name)?;

    let instance = module.instantiate(&mut store, Some(imports))?;
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
