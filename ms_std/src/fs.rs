use alloc::vec::Vec;
use ms_hostcall::types::{OpenFlags, OpenMode};

use crate::{
    io::{Read, Write},
    libos::libos,
};

pub struct File {
    raw_fd: u32,
}

impl File {
    pub fn create(p: &str) -> Result<Self, ()> {
        let flags = OpenFlags::O_CREAT;
        let mode = OpenMode::RDWR;
        let raw_fd = libos!(open(p, flags, mode)).expect("create file failed.");

        Ok(File { raw_fd })
    }

    pub fn open(p: &str) -> Result<Self, ()> {
        let mode = OpenMode::RDONLY;
        let flags = OpenFlags::empty();
        let raw_fd = libos!(open(p, flags, mode)).expect("open file failed.");

        Ok(File { raw_fd })
    }
}

impl Write for File {
    fn write_str(&mut self, _s: &str) -> core::fmt::Result {
        todo!()
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, crate::io::Error> {
        todo!()
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, crate::io::Error> {
        todo!()
    }
}
