use core::alloc::{Layout, LayoutError};

use alloc::string::String;
use bitflags::bitflags;
use thiserror_no_std::Error;

use crate::{mmap_file_backend::MmapFileErr, types::Fd};

bitflags! {
    #[derive(PartialEq, Eq)]
    pub struct ProtFlags: u32 {
        const READ = 1;
        const WRITE = 2;
        // const EXEC = 3;
    }
}

pub type BufferAllocFunc = fn(&str, Layout, u64) -> MMResult<usize>;
pub type AccessBufferFunc = fn(&str) -> Option<(usize, u64)>;
pub type BufferDeallocFunc = fn(usize, Layout);
pub type MemmapFunc = fn(usize, ProtFlags, Fd) -> MMResult<usize>;
pub type MemunmapFunc = fn(&mut [u8]) -> MMResult;
pub type HeapAllocFunc = fn(Layout) -> MMResult<*mut u8>;
pub type HeapDeallocFunc = fn(*mut u8, Layout) -> MMResult;

pub type MMResult<T = ()> = Result<T, MMError>;

#[derive(Debug, Error)]
pub enum MMError {
    #[error("invaild argument, expect: {0}, found: {1}")]
    InvaildArg(String, usize),
    #[error(transparent)]
    LayoutErr(#[from] LayoutError),
    #[error(transparent)]
    FileBackendErr(#[from] MmapFileErr),
    #[error("libc api error: {0}")]
    LibcErr(String),
    #[error("unknown")]
    Unknown,
}
