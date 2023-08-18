#![no_std]
#![allow(clippy::result_unit_err)]

extern crate alloc;

use alloc::vec::Vec;
use lazy_static::lazy_static;
use ms_hostcall::types::{OpenFlags, OpenMode};
use ms_std::{self, libos::libos};
use spin::Mutex;

enum DataSource {
    FatFS,
    _Net,
}

struct File {
    raw_fd: u32,
    mode: OpenMode,
    src: DataSource,
}

lazy_static! {
    static ref FD_TABLE: Mutex<Vec<File>> = Mutex::new(Vec::new());
}

#[no_mangle]
pub fn host_write(fd: i32, buf: &str) -> isize {
    match fd {
        1 | 2 => {
            libos!(stdout(buf));
            buf.len() as isize
        }
        _ => panic!(),
    }
}

#[no_mangle]
pub fn host_open(path: &str, flags: OpenFlags, mode: OpenMode) -> Result<u32, ()> {
    let file = {
        let fd = libos!(fatfs_open(path, flags)).expect("fatfs open file failed");
        File {
            raw_fd: fd,
            mode,
            src: DataSource::FatFS,
        }
    };

    let fd = {
        let mut fdtab = FD_TABLE.lock();
        fdtab.push(file);
        fdtab.len() + 2
    };

    Ok(fd as u32)
}
