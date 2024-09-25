#![no_std]

use ms_std::{agent::DataBuffer, println};
pub use tinywasm;
use tinywasm::{Extern, FuncContext, Imports, MemoryStringExt};

use ms_std_proc_macro::FaasData;

#[derive(FaasData)]
struct WasmDataBuffer(*mut u8, usize);

impl Default for WasmDataBuffer {
    fn default() -> Self {
        Self(core::ptr::null_mut(), Default::default())
    }
}

pub fn buffer_register(
    mut ctx: FuncContext,
    (slot_name_base, slot_name_size, buffer_base, buffer_size): (i32, i32, i32, i32),
) -> tinywasm::Result<()> {
    println!("buffer_register");

    let memory = ctx.exported_memory("memory").unwrap();
    let slot_name = memory
        .load_string(slot_name_base as usize, slot_name_size as usize)
        .unwrap();
    let mut wasm_buffer: DataBuffer<WasmDataBuffer> = DataBuffer::with_slot(slot_name);
    wasm_buffer.0 = buffer_base as usize as *mut _;
    wasm_buffer.1 = buffer_size as usize;

    Ok(())
}

pub fn access_buffer(
    mut ctx: FuncContext,
    (slot_name_base, slot_name_size, buffer_base, buffer_size): (i32, i32, i32, i32),
) -> tinywasm::Result<()> {
    let mut memory = ctx.exported_memory_mut("memory").unwrap();
    let slot_name = memory
        .load_string(slot_name_base as usize, slot_name_size as usize)
        .unwrap();

    let wasm_buffer: DataBuffer<WasmDataBuffer> = DataBuffer::from_buffer_slot(slot_name).unwrap();
    if buffer_size as usize != wasm_buffer.1 {
        panic!()
    }
    memory
        .store(buffer_base as usize, buffer_size as usize, unsafe {
            core::slice::from_raw_parts(wasm_buffer.0, wasm_buffer.1)
        })
        .unwrap();

    Ok(())
}

pub fn fd_close(mut _ctx: FuncContext, _args: i32) -> tinywasm::Result<i32> {
    Ok(0)
}

pub fn fd_fdstat_get(mut ctx: FuncContext<'_>, args: (i32, i32)) -> tinywasm::Result<i32> {
    Ok(0)
}

pub fn fd_seek(mut ctx: FuncContext<'_>, args: (i32, i64, i32, i32)) -> tinywasm::Result<i32> {
    Ok(0)
}

pub fn fd_write(mut ctx: FuncContext<'_>, args: (i32, i32, i32, i32)) -> tinywasm::Result<i32> {
    Ok(0)
}

pub fn proc_exit(_: FuncContext<'_>, _args: i32) -> tinywasm::Result<()> {
    panic!("normally exit");
}

pub fn import_all() -> tinywasm::Result<Imports> {
    let mut imports = Imports::new();

    imports.define(
        "env",
        "buffer_register",
        Extern::typed_func(buffer_register),
    )?;

    imports.define("env", "access_buffer", Extern::typed_func(access_buffer))?;

    imports.define(
        "wasi_snapshot_preview1",
        "fd_close",
        Extern::typed_func(fd_close),
    )?;

    imports.define(
        "wasi_snapshot_preview1",
        "fd_fdstat_get",
        Extern::typed_func(fd_fdstat_get),
    )?;

    imports.define(
        "wasi_snapshot_preview1",
        "fd_seek",
        Extern::typed_func(fd_seek),
    )?;

    imports.define(
        "wasi_snapshot_preview1",
        "fd_write",
        Extern::typed_func(fd_write),
    )?;

    imports.define(
        "wasi_snapshot_preview1",
        "proc_exit",
        Extern::typed_func(proc_exit),
    )?;

    Ok(imports)
}
