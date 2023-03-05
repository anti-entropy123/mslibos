pub mod types;

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
pub struct IsolationContext {
    pub isol_id: IsolationID,
    pub find_handler: usize,
}
