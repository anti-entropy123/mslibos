use alloc::string::String;
// use alloc::

use crate::{HostCallID, IsolationContext};

pub type IsolationID = u64;
pub type ServiceName = String;
pub type SymbolName = String;
pub type HostCallResult = Result<(), HostCallError>;

#[derive(Debug)]
pub enum HostCallError {
    HasBeenSet,
}

// msvisor
pub type FindHostCallFunc = unsafe extern "C" fn(IsolationID, HostCallID) -> usize;

// service init
pub type SetHandlerFunc = unsafe extern "C" fn(IsolationContext) -> HostCallResult;
pub type GetHandlerFunc = unsafe extern "C" fn() -> usize;
pub type PanicHandlerFunc = unsafe extern "C" fn() -> !;

// app main
pub type RustMainFunc = unsafe fn() -> ();

// fdtab
pub type HostWriteFunc = fn(i32, &str) -> isize;

// stdio
pub type HostStdioFunc = fn(&str) -> isize;

// socket
pub type SomltcpAddrInfoFunc = fn(&str) -> Result<core::net::Ipv4Addr, ()>;

pub trait Transmutor {
    fn find_host_call() -> FindHostCallFunc;
    fn host_panic_handler() -> PanicHandlerFunc;

    fn host_write_func(&mut self) -> HostWriteFunc;
    fn host_stdio_func(&mut self) -> HostStdioFunc;
    fn somltcp_addrinfo(&mut self) -> SomltcpAddrInfoFunc;
}
