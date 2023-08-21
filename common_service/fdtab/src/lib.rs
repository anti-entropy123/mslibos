#![no_std]
#![allow(clippy::result_unit_err)]

extern crate alloc;

use alloc::vec::Vec;
use lazy_static::lazy_static;
use ms_hostcall::types::{Fd, OpenFlags, OpenMode, Size};
use ms_std::{self, libos::libos};
use spin::Mutex;

#[derive(Clone)]
enum DataSource {
    FatFS,
    _Net,
}

#[derive(Clone)]
struct File {
    raw_fd: Fd,
    mode: OpenMode,
    src: DataSource,
}

lazy_static! {
    static ref FD_TABLE: Mutex<Vec<Option<File>>> = Mutex::new(Vec::new());
}

fn get_file(fd: Fd) -> File {
    let fdtab = FD_TABLE.lock();
    // println!("get_file: fd={}, fd_table.len()={}", fd, fdtab.len());
    if let Some(Some(file)) = fdtab.get(fd as usize - 3) {
        file.clone()
    } else {
        panic!("file fd={} not exist?", fd)
    }
}

#[no_mangle]
pub fn host_read(fd: Fd, buf: &mut [u8]) -> Result<Size, ()> {
    match fd {
        0 => unimplemented!(),
        1 | 2 => Err(()),
        _ => {
            let file = get_file(fd);
            if file.mode == OpenMode::WRONLY {
                return Err(());
            }

            match file.src {
                DataSource::FatFS => {
                    Ok(libos!(fatfs_read(file.raw_fd, buf)).expect("fatfs read failed."))
                }
                DataSource::_Net => todo!(),
            }
        }
    }
}

#[no_mangle]
pub fn host_write(fd: Fd, buf: &[u8]) -> Result<Size, ()> {
    match fd {
        0 => unimplemented!(),
        1 | 2 => {
            libos!(stdout(buf));
            Ok(buf.len())
        }
        _ => {
            let file = get_file(fd);
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
pub fn host_open(path: &str, flags: OpenFlags, mode: OpenMode) -> Result<Fd, ()> {
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
        fdtab.push(Some(file));
        fdtab.len() + 2
    };

    Ok(fd as Fd)
}

#[no_mangle]
pub fn host_close(fd: Fd) -> Result<(), ()> {
    match fd {
        0 | 1 | 2 => Err(()),
        _ => {
            let file = get_file(fd);
            let mut fdtab = FD_TABLE.lock();
            fdtab[fd as usize - 3] = None;

            match file.src {
                DataSource::FatFS => {
                    Ok(libos!(fatfs_close(file.raw_fd)).expect("fatfs read failed."))
                }
                DataSource::_Net => todo!(),
            }
        }
    }
}
