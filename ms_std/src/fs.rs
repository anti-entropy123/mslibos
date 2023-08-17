use alloc::vec::Vec;

use crate::io::{Read, Write};

pub struct File;

impl File {
    pub fn create(_p: &str) -> Result<Self, ()> {
        todo!()
    }

    pub fn open(_p: &str) -> Result<Self, ()> {
        todo!()
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
