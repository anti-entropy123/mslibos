#![no_std]

pub use tinywasm;
use tinywasm::{Extern, Imports};

mod data_buffer;
mod wasi;

pub fn import_all() -> tinywasm::Result<Imports> {
    let mut imports = Imports::new();

    imports
        .define(
            "env",
            "buffer_register",
            Extern::typed_func(data_buffer::buffer_register),
        )?
        .define(
            "env",
            "access_buffer",
            Extern::typed_func(data_buffer::access_buffer),
        )?
        .define(
            "wasi_snapshot_preview1",
            "fd_close",
            Extern::typed_func(wasi::fd_close),
        )?
        .define(
            "wasi_snapshot_preview1",
            "fd_fdstat_get",
            Extern::typed_func(wasi::fd_fdstat_get),
        )?
        .define(
            "wasi_snapshot_preview1",
            "fd_fdstat_set_flags",
            Extern::typed_func(wasi::fd_fdstat_set_flags),
        )?
        .define(
            "wasi_snapshot_preview1",
            "fd_prestat_get",
            Extern::typed_func(wasi::fd_prestat_get),
        )?
        .define(
            "wasi_snapshot_preview1",
            "fd_prestat_dir_name",
            Extern::typed_func(wasi::fd_prestat_dir_name),
        )?
        .define(
            "wasi_snapshot_preview1",
            "fd_read",
            Extern::typed_func(wasi::fd_read),
        )?
        .define(
            "wasi_snapshot_preview1",
            "fd_read",
            Extern::typed_func(wasi::fd_read),
        )?
        .define(
            "wasi_snapshot_preview1",
            "fd_seek",
            Extern::typed_func(wasi::fd_seek),
        )?
        .define(
            "wasi_snapshot_preview1",
            "fd_write",
            Extern::typed_func(wasi::fd_write),
        )?
        .define(
            "wasi_snapshot_preview1",
            "path_open",
            Extern::typed_func(wasi::path_open),
        )?
        .define(
            "wasi_snapshot_preview1",
            "proc_exit",
            Extern::typed_func(wasi::proc_exit),
        )?;

    Ok(imports)
}
