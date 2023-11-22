use ms_hostcall::types::{Fd, OpenFlags, OpenMode, Stat};

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
        let mode = OpenMode::RD;
        let flags = OpenFlags::empty();
        let raw_fd = libos!(open(p, flags, mode)).expect("open file failed.");

        Ok(File { raw_fd })
    }

    pub fn seek(&mut self, pos: u32) {
        libos!(lseek(self.raw_fd, pos)).expect("seek failed.")
    }

    pub fn as_raw_fd(&self) -> Fd {
        self.raw_fd
    }

    pub fn from_raw_fd(fd: Fd) -> Self {
        Self { raw_fd: fd }
    }

    pub fn metadata(&self) -> Result<Stat, ()> {
        libos!(stat(self.raw_fd))
    }
}

impl Write for File {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        if libos!(write(self.raw_fd, s.as_bytes())).is_err() {
            Err(core::fmt::Error)
        } else {
            Ok(())
        }
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, crate::io::Error> {
        libos!(read(self.raw_fd, buf)).map_err(|_| ())
    }
}

impl Drop for File {
    fn drop(&mut self) {
        libos!(close(self.raw_fd)).expect("File::drop close file failed.")
    }
}
