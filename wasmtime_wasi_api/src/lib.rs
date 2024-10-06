#![no_std]

pub use wasmtime;
use wasmtime::Linker;

mod data_buffer;
mod wasi;
mod errno;

pub fn import_all(linker: &mut Linker<()>) {
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
            "proc_exit",
            wasi::proc_exit,
        )
        .unwrap();

}