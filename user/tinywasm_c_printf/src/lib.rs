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
use ms_std::libos::libos;
use ms_hostcall::types::{OpenFlags, OpenMode};

use tinywasm::{Extern, MemoryStringExt};
use tinywasm::{FuncContext, Imports, Module, Store};

const WASM: &[u8] = include_bytes!("../write.wasm");

#[repr(C)]
struct WasiCiovec {
    buf: u32,
    buf_len: u32,
}

#[repr(C)]
struct WasiFdstat {
    fs_filetype: u8,
    fs_flags: u16,
    fs_rights_base: u64,
    fs_rights_inheriting: u64,
}

#[no_mangle]
pub fn main(_args: &BTreeMap<String, String>) -> Result<()> {
    let module = Module::parse_bytes(WASM)?;
    let mut store = Store::default();
    let mut imports = Imports::new();

    let fd_close= Extern::typed_func(|_: FuncContext<'_>, _args: i32| -> tinywasm::Result<i32> {
        println!("[Debug] Invoke into fd_close");
        println!("args: {:?}", _args);

        let fd = _args as u32;
        libos!(close(fd)).unwrap();

        Ok(0)
    });

    let fd_fdstat_get = Extern::typed_func(
        |mut ctx: FuncContext<'_>, args: (i32, i32)| -> tinywasm::Result<i32> {
            println!("[Debug] Invoke into fd_fdstat_get");
            println!("args: {:?}, {:?}", args.0, args.1);

            let fd = args.0 as u32;
            let addr = args.1 as usize;
            let mut mem = ctx.exported_memory_mut("memory")?;

            /* fake implementation */

            let mut fdstat = WasiFdstat {
                fs_filetype: 0, 
                fs_flags: 0,    
                fs_rights_base: 0xFFFFFFFFFFFFFFFF, 
                fs_rights_inheriting: 0xFFFFFFFFFFFFFFFF, 
            };

            if fd == 1 || fd == 2 {
                fdstat.fs_flags = 1;
                fdstat.fs_filetype = 0b00000100;
            }

            let ret = (&fdstat) as *const _ as usize;
            let ret = unsafe { core::slice::from_raw_parts(ret as *const u8, core::mem::size_of::<WasiFdstat>()) } ;
            mem.store(addr,core::mem::size_of::<WasiFdstat>(), ret)?;

            Ok(0)
        },
    );

    let fd_seek = Extern::typed_func(
        |_: FuncContext<'_>, args: (i32, i64, i32, i32)| -> tinywasm::Result<i32> {
            println!("[Debug] Invoke into fd_seek");
            println!("args: {:?}, {:?}, {:?}, {:?}", args.0, args.1, args.2, args.3);

            let fd = args.0 as u32;
            let offset = args.1;
            let whence = args.2;
            let pos = offset as u32;
            // if whence == 0 {
                
            // } else if whence == 1 {
                
            // } else if whence == 2{
                
            // }

            libos!(lseek(fd, pos)).unwrap();
            
            Ok(0)
        },
    );

    let fd_write = Extern::typed_func(
        |mut ctx: FuncContext<'_>, args: (i32, i32, i32, i32)| -> tinywasm::Result<i32> {
            println!("[Debug] Invoke into fd_write");
            println!("args: {:?}, {:?}, {:?}, {:?}", args.0, args.1, args.2, args.3);

            let fd = args.0 as u32;
            let ptr = args.1 as usize;
            let iovs_len = args.2 as usize;
            let mut mem = ctx.exported_memory_mut("memory")?;
            let mut ret: usize = 0;

            for i in 0..iovs_len {
                let offset = ptr + i * core::mem::size_of::<WasiCiovec>();
                let iovs: &[u8] = mem.load(offset, core::mem::size_of::<WasiCiovec>())?;
                let iovs: &WasiCiovec = unsafe { &*(iovs.as_ptr() as *const WasiCiovec) };

                println!("i: {:?}, offset: {:?}, addr.buf: {:?}, addr.buf_len: {:?}", i, offset, iovs.buf, iovs.buf_len);
                let buf = mem.load(iovs.buf as usize, iovs.buf_len as usize)? as &[u8];
                ret += libos!(write(fd, buf)).unwrap();
            }
            
            println!("ret size: {:?}", ret);
            mem.store(args.3 as usize, core::mem::size_of::<usize>(), &ret.to_ne_bytes())?;

            Ok(0)
        },
    );

    let proc_exit = Extern::typed_func(|_: FuncContext<'_>, _args: i32| -> tinywasm::Result<()> {
        println!("[Debug] Invoke into proc_exit");

        panic!("normally exit");
        Ok(())
    });

    let fd_read = Extern::typed_func(
        |mut ctx: FuncContext<'_>, args: (i32, i32, i32, i32)| -> tinywasm::Result<i32> {
            println!("[Debug] Invoke into fd_read\n");

            let fd = args.0 as u32;
            let ptr = args.1 as usize;
            let iovs_len = args.2 as usize;
            let mut mem = ctx.exported_memory_mut("memory")?;
            let mut read_size: usize = 0;

            for i in 0..iovs_len {
                let offset = ptr + i * core::mem::size_of::<WasiCiovec>();
                let addr: &[u8] = mem.load(offset, core::mem::size_of::<WasiCiovec>())?;
                let addr: &WasiCiovec = unsafe { &*(addr.as_ptr() as *const WasiCiovec) };
                let buf: &[u8] = mem.load(addr.buf as usize, addr.buf_len as usize)?;
                let buf: &mut [u8] = unsafe { slice::from_raw_parts_mut(buf.as_ptr() as usize as *mut u8, addr.buf_len as usize) };
                read_size += libos!(read(fd, buf)).unwrap();
            }

            mem.store(args.3 as usize, core::mem::size_of::<u32>(), &read_size.to_ne_bytes() )?;

            Ok(0)
        },
    );

    let path_open = Extern::typed_func(
        |mut ctx: FuncContext<'_>, args: (i32, i32, i32, i32, i32, i64, i64, i32, i32)| -> tinywasm::Result<i32> {
            println!("[Debug] Invoke into path_open\n");

            let mem = ctx.exported_memory("memory")?;
            let fd = args.0 as u32;
            let dirflags = args.1 as u32;
            let path_addr = args.2 as u32;
            let path_len = args.3 as u32;
            let path = mem.load_string(path_addr as usize, path_len as usize)?;
            let oflags = args.4 as u16;
            
            let fs_rights_base = args.5 as u64;
            let fs_rights_inheriting = args.6 as u64;
            let fdflags = args.7 as u16;
            
            let mut flags: OpenFlags = OpenFlags::empty();
            if (fdflags & (1 << 0)) == 1 {
                flags |= OpenFlags::O_APPEND;
            }
            if (oflags & (1 << 0)) == 1 {
                flags |= OpenFlags::O_CREAT;
            }

            let mut mode: OpenMode = OpenMode::empty();
            if (fs_rights_base & (1 << 1)) == 1 {
                mode |= OpenMode::RD;
            }
            if (fs_rights_base & (1 << 6)) == 1 {
                mode |= OpenMode::WR;
            }

            let ret_fd = libos!(open(&path, flags, mode)).unwrap() as i32;
            Ok(ret_fd)
        },
    );

    let fd_fdstat_set_flags = Extern::typed_func(
        |_: FuncContext<'_>, args: (i32, i32)| -> tinywasm::Result<i32> {
            println!("[Debug] Invoke into fd_fdstat_set_flags\n");

            let fd = args.0 as u32;
            let flag = args.1 as u16;

            Ok(0)
        },
    );

    let fd_prestat_get = Extern::typed_func(
        |_: FuncContext<'_>, args: (i32, i32)| -> tinywasm::Result<i32> {
            println!("[Debug] Invoke into fd_prestat_get\n");

            Ok(0)
        },
    );

    let fd_prestat_dir_name = Extern::typed_func(
        |_: FuncContext<'_>, args: (i32, i32, i32)| -> tinywasm::Result<i32> {
            println!("[Debug] Invoke into fd_prestat_dir_name\n");

            Ok(0)
        },
    );

    imports
        .define("wasi_snapshot_preview1", "fd_close", fd_close)?
        .define("wasi_snapshot_preview1", "fd_fdstat_get", fd_fdstat_get)?
        .define("wasi_snapshot_preview1", "fd_fdstat_set_flags", fd_fdstat_set_flags)?
        .define("wasi_snapshot_preview1", "fd_prestat_get", fd_prestat_get)?
        .define("wasi_snapshot_preview1", "fd_prestat_dir_name", fd_prestat_dir_name)?
        .define("wasi_snapshot_preview1", "fd_read", fd_read)?
        .define("wasi_snapshot_preview1", "fd_seek", fd_seek)?
        .define("wasi_snapshot_preview1", "fd_write", fd_write)?
        .define("wasi_snapshot_preview1", "path_open", path_open)?
        .define("wasi_snapshot_preview1", "proc_exit", proc_exit)?;

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
