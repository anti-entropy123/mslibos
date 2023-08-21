#![no_std]
#![allow(clippy::result_unit_err)]

extern crate alloc;

use alloc::vec::Vec;
use lazy_static::lazy_static;
use ms_hostcall::types::{Fd, OpenFlags, OpenMode, Size};
use ms_std::{self, libos::libos, println};
use spin::Mutex;

enum DataSource {
    FatFS,
    _Net,
}

struct File {
    raw_fd: Fd,
    mode: OpenMode,
    src: DataSource,
}

lazy_static! {
    static ref FD_TABLE: Mutex<Vec<File>> = Mutex::new(Vec::new());
}

#[no_mangle]
pub fn host_write(fd: Fd, buf: &str) -> Result<Size, ()> {
    match fd {
        1 | 2 => {
            libos!(stdout(buf));
            Ok(buf.len())
        }
        _ => {
            let fdtab = FD_TABLE.lock();
            println!("host_write: fd={}, fd_table.len()={}", fd, fdtab.len());
            let file = fdtab
                .get(fd as usize - 3)
                .unwrap_or_else(|| panic!("file fd={} not exist?", fd));

            if file.mode == OpenMode::RDONLY {
                return Err(());
            }

            match file.src {
                DataSource::FatFS => {
                    Ok(libos!(fatfs_write(file.raw_fd, buf)).expect("fatfs write failed."))
                }
                DataSource::_Net => todo!(),
            }
        }
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
