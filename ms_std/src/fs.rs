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
        let _size = libos!(write(self.raw_fd, s.as_bytes())).expect("write failed.");

        Ok(())
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, crate::io::Error> {
        libos!(read(self.raw_fd, buf))
    }
}

impl Drop for File {
    fn drop(&mut self) {
        libos!(close(self.raw_fd)).expect("File::drop close file failed.")
    }
}
