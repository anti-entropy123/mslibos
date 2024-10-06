#![no_std]

extern crate alloc;

use alloc::{string::{String, ToString}, vec::Vec};
pub use wasmtime;
use wasmtime::{Engine, Linker, Module};

mod data_buffer;
mod wasi;
mod types;

pub struct LibosCtx {
    pub id: String
}

pub fn build_wasm(cwasm: &[u8]) -> (Engine, Module, Linker<LibosCtx>) {
    let engine: Engine = Engine::default();
    let module: Module = unsafe { Module::deserialize(&engine, cwasm) }.map_err(|e| e.to_string()).unwrap();
    let mut linker = Linker::new(&engine);
    import_all(&mut linker);
    (engine, module, linker)
}

pub fn set_wasi_args(id: &str, args: Vec<String>) {
    wasi::set_wasi_state(id, args);
}

fn import_all(linker: &mut Linker<LibosCtx>) {
    linker
        .func_wrap(
            "env",
            "buffer_register",
            data_buffer::buffer_register,
        )
        .unwrap();
    linker
        .func_wrap(
            "env",
            "access_buffer",
            data_buffer::access_buffer,
        )
        .unwrap();
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "args_get",
            wasi::args_get,
        )
        .unwrap();
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "args_sizes_get",
            wasi::args_sizes_get,
        )
        .unwrap();
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "fd_close",
            wasi::fd_close,
        )
        .unwrap();
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "fd_fdstat_get",
            wasi::fd_fdstat_get,
        )
        .unwrap();
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "fd_fdstat_set_flags",
            wasi::fd_fdstat_set_flags,
        )
        .unwrap();
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "fd_prestat_get",
            wasi::fd_prestat_get,
        )
        .unwrap();
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "fd_prestat_dir_name",
            wasi::fd_prestat_dir_name,
        )
        .unwrap();
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "fd_read",
            wasi::fd_read,
        )
        .unwrap();
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "fd_seek",
            wasi::fd_seek,
        )
        .unwrap();
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "fd_write",
            wasi::fd_write,
        )
        .unwrap();
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "path_open",
            wasi::path_open,
        )
        .unwrap();
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "proc_exit",
            wasi::proc_exit,
        )
        .unwrap();

}