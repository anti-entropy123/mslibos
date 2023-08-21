use alloc::vec::Vec;
use ms_hostcall::types::{Fd, OpenFlags, OpenMode};

use crate::{
    io::{Read, Write},
    libos::libos,
};

pub struct File {
    raw_fd: Fd,
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
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let _size = libos!(write(self.raw_fd, s.as_bytes())).expect("");

        Ok(())
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, crate::io::Error> {
        libos!(read(self.raw_fd, buf))
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, crate::io::Error> {
        let mut size = 0;
        let mut buffer = [0; 1024];

        loop {
            let s = self.read(&mut buffer).expect("File::read failed.");
            // println!("read {} bytes, total {} bytes.", s, size);
            if s == 0 {
                break;
            }
            size += s;

            buf.extend_from_slice(&buffer[0..s])
        }

        Ok(size)
    }
}

impl Drop for File {
    fn drop(&mut self) {
        libos!(close(self.raw_fd)).expect("File::drop close file failed.")
    }
}
