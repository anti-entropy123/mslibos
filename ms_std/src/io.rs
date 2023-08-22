pub use core::fmt::Write;

use alloc::vec::Vec;

pub type Error = ();
pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error>;
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, Error>;
}
