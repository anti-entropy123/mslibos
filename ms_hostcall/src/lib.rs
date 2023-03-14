#![no_std]

extern crate alloc;

pub mod types;

use alloc::string::String;
use types::{FindHostCallFunc, HostWriteFunc, IsolationID};

#[derive(Debug)]
#[repr(C)]
pub enum CommonHostCall {
    Write,
}

#[derive(Debug)]
#[repr(C)]
pub enum HostCallID {
    Common(CommonHostCall),
    Custom(String),
}

pub trait Transmutor {
    fn host_write_func(&self) -> HostWriteFunc;
    fn find_host_call(&self) -> FindHostCallFunc;
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct IsolationContext {
    pub isol_id: IsolationID,
    pub find_handler: usize,
    pub heap_range: (usize, usize),
}

pub const SERVICE_HEAP_SIZE: usize = 4096 * 8;
