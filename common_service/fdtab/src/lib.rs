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
    static ref FD_TABLE: FdTable = FdTable::new();
}

struct FdTable {
    inner: Mutex<Vec<Option<File>>>,
}

impl FdTable {
    fn new() -> Self {
        FdTable {
            inner: Mutex::default(),
        }
    }

    fn with_file<F, T>(&self, fd: Fd, f: F) -> T
    where
        F: FnOnce(Option<&File>) -> T,
    {
        let fdtab = self.inner.lock();
        // println!("get_file: fd={}, fd_table.len()={}", fd, fdtab.len());
        let file = if let Some(Some(file)) = fdtab.get(fd as usize - 3) {
            Some(file)
        } else {
            println!("file fd={} not exist?", fd);
            None
        };

        f(file)
    }

    fn with_file_mut<F, T>(&self, fd: Fd, f: F) -> T
    where
        F: FnOnce(Option<&mut File>) -> T,
    {
        // println!("get_file: fd={}, fd_table.len()={}", fd, fdtab.len());
        let mut fdtab = self.inner.lock();
        let file = if let Some(Some(file)) = fdtab.get_mut(fd as usize - 3) {
            Some(file)
        } else {
            println!("file fd={} not exist?", fd);
            None
        };

        f(file)
    }

    fn add_file(&self, file: File) -> Fd {
        let mut fdtab = self.inner.lock();
        for (idx, item) in fdtab.iter_mut().enumerate() {
            if item.is_some() {
                continue;
            }

            *item = Some(file);
            return (idx + 3) as Fd;
        }

        fdtab.push(Some(file));
        (fdtab.len() + 2) as Fd
    }

    fn remove_file(&self, fd: Fd) -> Option<File> {
        let mut fdtab = self.inner.lock();
        if fd as usize - 3 >= fdtab.len() {
            return None;
        }

        let old = fdtab[fd as usize - 3].clone();
        fdtab[fd as usize - 3] = None;

        old
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

    FD_TABLE.with_file(fd, |file| {
        let file = if let Some(file) = file {
            file
        } else {
            return Err(LibOSErr::BadFileDescriptor);
        };

        if !file.can_read() {
            return Err(LibOSErr::NoReadPerm);
        }

        match file.src {
            DataSource::FatFS(raw_fd) => libos!(fatfs_read(raw_fd, buf)),
            DataSource::Net(socket) => libos!(recv(socket, buf)),
        }
        .map_err(|_| LibOSErr::Unknown)
    })
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

    FD_TABLE.with_file(fd, |file| {
        let file = if let Some(file) = file {
            file
        } else {
            return Err(LibOSErr::BadFileDescriptor);
        };

        if !file.can_write() {
            return Err(LibOSErr::NoWritePerm);
        }

        match file.src {
            DataSource::FatFS(raw_fd) => {
                libos!(fatfs_write(raw_fd, buf)).map_err(|_| LibOSErr::Unknown)
            }
            DataSource::Net(sockfd) => {
                if libos!(send(sockfd, buf)).is_err() {
                    Err(LibOSErr::Unknown)
                } else {
                    Ok(buf.len())
                }
            }
        }
    })
}

#[no_mangle]
pub fn lseek(fd: Fd, pos: u32) -> LibOSResult<()> {
    if let 0..=2 = fd {
        return Err(LibOSErr::BadFileDescriptor);
    }
    FD_TABLE.with_file(fd, |file| {
        let file = if let Some(file) = file {
            file
        } else {
            return Err(LibOSErr::BadFileDescriptor);
        };

        match file.src {
            DataSource::FatFS(raw_fd) => {
                libos!(fatfs_seek(raw_fd, pos)).expect("fatfs seek failed.")
            }
            DataSource::Net(_) => panic!("The file type does not match"),
        };

        Ok(())
    })
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

    Ok(FD_TABLE.add_file(file))
}

#[no_mangle]
pub fn close(fd: Fd) -> Result<(), ()> {
    if (0..=2).contains(&fd) {
        return Err(());
    };

    let file = match FD_TABLE.remove_file(fd) {
        Some(f) => f,
        None => return Err(()),
    };

    match file.src {
        DataSource::FatFS(raw_fd) => Ok(libos!(fatfs_close(raw_fd))?),
        DataSource::Net(socket) => libos!(smol_close(socket)).map_err(|_| ()),
    }
}

#[no_mangle]
pub fn connect(addr: SocketAddrV4) -> Result<SockFd, ()> {
    libos!(smol_connect(addr)).map(|sockfd| {
        let file = File {
            mode: OpenMode::RDWR,
            src: DataSource::Net(sockfd),
        };

        FD_TABLE.add_file(file)
    })
}

#[no_mangle]
pub fn bind(addr: SocketAddrV4) -> LibOSResult<SockFd> {
    libos!(smol_bind(addr)).map(|listened_sockfd| {
        let file = File {
            mode: OpenMode::RD,
            src: DataSource::Net(listened_sockfd),
        };

        FD_TABLE.add_file(file)
    })
}

#[no_mangle]
pub fn accept(listened_sockfd: SockFd) -> LibOSResult<SockFd> {
    if (0..=2).contains(&listened_sockfd) {
        return Err(LibOSErr::BadFileDescriptor);
    };

    let listened_sockfd = FD_TABLE.with_file_mut(listened_sockfd, |file| {
        let old_sock = match file {
            None => return Err(LibOSErr::BadFileDescriptor),
            Some(f) => f,
        };

        let listened_sockfd = if let DataSource::Net(sockfd) = old_sock.src {
            // println!("sockfd is {}", sockfd);
            sockfd
        } else {
            return Err(LibOSErr::BadFileDescriptor);
        };

        // old file is still listened socket, with new socket handle.
        old_sock.src = DataSource::Net(libos!(smol_accept(listened_sockfd))?);
        Ok(listened_sockfd)
    })?;

    // new file will be connected socket, with old socket handle.
    let new_sock = File {
        mode: OpenMode::RDWR,
        src: DataSource::Net(listened_sockfd),
    };
    let connected_sockfd = FD_TABLE.add_file(new_sock);

    Ok(connected_sockfd)
}
