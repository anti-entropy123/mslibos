pub mod types;

use types::{FindHostCallFunc, HostWriteFunc};

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
