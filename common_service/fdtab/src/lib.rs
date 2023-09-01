#![no_std]
#![feature(ip_in_core)]
#![allow(clippy::result_unit_err)]

extern crate alloc;

use core::net::SocketAddrV4;

use alloc::vec::Vec;

use lazy_static::lazy_static;
use spin::Mutex;

use ms_hostcall::{
    err::{LibOSErr, LibOSResult},
    types::{Fd, OpenFlags, OpenMode, Size, SockFd},
};
use ms_std::{self, libos::libos, println};

#[derive(Clone)]
enum DataSource {
    FatFS(Fd),
    Net(SockFd),
}

#[derive(Clone)]
struct File {
    mode: OpenMode,
    src: DataSource,
}

impl File {
    fn can_read(&self) -> bool {
        // OpenMode::RDWR => 011;
        // OpenMode::RD   => 001;
        self.mode.contains(OpenMode::RD)
    }

    fn can_write(&self) -> bool {
        self.mode.contains(OpenMode::WR)
    }
}

lazy_static! {
    static ref FD_TABLE: Mutex<Vec<Option<File>>> = Mutex::new(Vec::new());
}

fn add_new_file(fdtab: &mut Vec<Option<File>>, file: File) -> Fd {
    for (idx, item) in fdtab.iter_mut().enumerate() {
        if item.is_some() {
            continue;
        }

        *item = Some(file);
        return (idx + 2) as Fd;
    }

    fdtab.push(Some(file));
    (fdtab.len() + 2) as Fd
}

fn get_file(fd: Fd) -> Option<File> {
    let fdtab = FD_TABLE.lock();
    // println!("get_file: fd={}, fd_table.len()={}", fd, fdtab.len());
    if let Some(Some(file)) = fdtab.get(fd as usize - 3) {
        Some(file.clone())
    } else {
        println!("file fd={} not exist?", fd);
        None
    }
}

fn get_file_mut(fdtab: &mut [Option<File>], fd: Fd) -> Option<&mut File> {
    // println!("get_file: fd={}, fd_table.len()={}", fd, fdtab.len());
    if let Some(Some(file)) = fdtab.get_mut(fd as usize - 3) {
        Some(file)
    } else {
        println!("file fd={} not exist?", fd);
        None
    }
}

#[no_mangle]
pub fn read(fd: Fd, buf: &mut [u8]) -> LibOSResult<Size> {
    match fd {
        // Maybe stdin can be implemented.
        0 => unimplemented!(),
        1 | 2 => return Err(LibOSErr::BadFileDescriptor),
        _ => {}
    }

    let file = match get_file(fd) {
        Some(f) => f,
        None => return Err(LibOSErr::BadFileDescriptor),
    };

    if !file.can_read() {
        return Err(LibOSErr::NoReadPerm);
    }

    match file.src {
        DataSource::FatFS(raw_fd) => libos!(fatfs_read(raw_fd, buf)),
        DataSource::Net(socket) => libos!(recv(socket, buf)),
    }
    .map_err(|_| LibOSErr::Unknown)
}

#[no_mangle]
pub fn write(fd: Fd, buf: &[u8]) -> LibOSResult<Size> {
    match fd {
        0 => return Err(LibOSErr::BadFileDescriptor),
        1 | 2 => {
            libos!(stdout(buf));
            return Ok(buf.len());
        }
        _ => {}
    }

    let file = match get_file(fd) {
        Some(f) => f,
        None => return Err(LibOSErr::BadFileDescriptor),
    };

    if !file.can_write() {
        return Err(LibOSErr::NoWritePerm);
    }

    match file.src {
        DataSource::FatFS(raw_fd) => {
            libos!(fatfs_write(raw_fd, buf)).map_err(|_| LibOSErr::Unknown)
        }
        DataSource::Net(socket) => {
            if libos!(send(socket, buf)).is_err() {
                Err(LibOSErr::Unknown)
            } else {
                Ok(buf.len())
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

    Ok(add_new_file(&mut FD_TABLE.lock(), file))
}

#[no_mangle]
pub fn close(fd: Fd) -> Result<(), ()> {
    if (0..=2).contains(&fd) {
        return Err(());
    };

    let file = match get_file(fd) {
        Some(f) => f,
        None => return Err(()),
    };

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

#[no_mangle]
pub fn connect(addr: SocketAddrV4) -> Result<SockFd, ()> {
    libos!(smol_connect(addr)).map(|sockfd| {
        let file = File {
            mode: OpenMode::RDWR,
            src: DataSource::Net(sockfd),
        };

        add_new_file(&mut FD_TABLE.lock(), file)
    })
}

#[no_mangle]
pub fn bind(addr: SocketAddrV4) -> LibOSResult<SockFd> {
    libos!(smol_bind(addr)).map(|listened_sockfd| {
        let file = File {
            mode: OpenMode::RD,
            src: DataSource::Net(listened_sockfd),
        };

        add_new_file(&mut FD_TABLE.lock(), file)
    })
}

#[no_mangle]
pub fn accept(listened_sockfd: SockFd) -> LibOSResult<SockFd> {
    if (0..=2).contains(&listened_sockfd) {
        return Err(LibOSErr::BadFileDescriptor);
    };

    let mut fdtab = FD_TABLE.lock();

    let old_sock = match get_file_mut(&mut fdtab, listened_sockfd) {
        None => return Err(LibOSErr::BadFileDescriptor),
        Some(f) => f,
    };

    let listened_sockfd = if let DataSource::Net(sockfd) = old_sock.src {
        println!("sockfd is {}", sockfd);
        sockfd
    } else {
        return Err(LibOSErr::BadFileDescriptor);
    };

    match libos!(smol_accept(listened_sockfd)) {
        // old file is still listened socket, with new socket handle.
        Ok(sockfd) => old_sock.src = DataSource::Net(sockfd),
        Err(e) => return Err(e),
    };

    // new file will be connected socket, with old socket handle.
    let new_sock = File {
        mode: OpenMode::RDWR,
        src: DataSource::Net(listened_sockfd),
    };
    let connected_sockfd = add_new_file(&mut fdtab, new_sock);

    Ok(connected_sockfd)
}
