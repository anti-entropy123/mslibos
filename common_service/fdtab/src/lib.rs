#![no_std]
#![feature(ip_in_core)]
#![allow(clippy::result_unit_err)]

extern crate alloc;

use core::net::SocketAddrV4;

use alloc::vec::Vec;
use lazy_static::lazy_static;
use ms_hostcall::types::{Fd, OpenFlags, OpenMode, Size, Socket};
use ms_std::{self, libos::libos};
use spin::Mutex;

#[derive(Clone)]
enum DataSource {
    FatFS(Fd),
    Net(Socket),
}

#[derive(Clone)]
struct File {
    mode: OpenMode,
    src: DataSource,
}

lazy_static! {
    static ref FD_TABLE: Mutex<Vec<Option<File>>> = Mutex::new(Vec::new());
}

fn add_new_file(file: File) -> Fd {
    let mut fdtab = FD_TABLE.lock();
    fdtab.push(Some(file));
    (fdtab.len() + 2) as Fd
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
pub fn read(fd: Fd, buf: &mut [u8]) -> Result<Size, ()> {
    match fd {
        0 => unimplemented!(),
        1 | 2 => Err(()),
        _ => {
            let file = get_file(fd);
            if file.mode == OpenMode::WRONLY {
                return Err(());
            }

            match file.src {
                DataSource::FatFS(raw_fd) => {
                    Ok(libos!(fatfs_read(raw_fd, buf)).expect("fatfs read failed."))
                }
                DataSource::Net(socket) => {
                    libos!(recv(socket, buf))
                }
            }
        }
    }
}

#[no_mangle]
pub fn write(fd: Fd, buf: &[u8]) -> Result<Size, ()> {
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
                DataSource::FatFS(raw_fd) => {
                    Ok(libos!(fatfs_write(raw_fd, buf)).expect("fatfs write failed."))
                }
                DataSource::Net(socket) => libos!(send(socket, buf)).map(|_| buf.len()),
            }
        }
    }
}

#[no_mangle]
pub fn open(path: &str, flags: OpenFlags, mode: OpenMode) -> Result<Fd, ()> {
    let file = {
        let raw_fd = libos!(fatfs_open(path, flags)).expect("fatfs open file failed");
        File {
            mode,
            src: DataSource::FatFS(raw_fd),
        }
    };

    Ok(add_new_file(file))
}

#[no_mangle]
pub fn close(fd: Fd) -> Result<(), ()> {
    match fd {
        0..=2 => Err(()),
        _ => {
            let file = get_file(fd);
            let mut fdtab = FD_TABLE.lock();
            fdtab[fd as usize - 3] = None;

            match file.src {
                DataSource::FatFS(raw_fd) => {
                    libos!(fatfs_close(raw_fd)).expect("fatfs read failed.");
                    Ok(())
                }
                DataSource::Net(_socket) => todo!(),
            }
        }
    }
}

#[no_mangle]
pub fn connect(addr: SocketAddrV4) -> Result<Fd, ()> {
    libos!(smol_connect(addr)).map(|socket| {
        let file = File {
            mode: OpenMode::RDWR,
            src: DataSource::Net(socket),
        };

        add_new_file(file)
    })
}
