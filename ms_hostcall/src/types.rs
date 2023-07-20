use core::{alloc::Layout, net::SocketAddrV4};

use alloc::string::String;
// use smoltcp::{iface::Interface, phy::TunTapInterface};

use crate::{HostCallID, IsolationContext};

pub type IsolationID = u64;
pub type ServiceName = String;
pub type SymbolName = String;
pub type HostCallResult = Result<(), HostCallError>;
pub struct NetdevName {
    pub name: String, // like "tap-7a323b"
    pub subnet: core::net::Ipv4Addr,
    pub mask: u8,
}

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

// service drop
pub type DropHandlerFunc = unsafe fn();

// app main
pub type RustMainFunc = unsafe fn() -> Result<(), ()>;

// fdtab
pub type HostWriteFunc = fn(i32, &str) -> isize;

// stdio
pub type HostStdioFunc = fn(&str) -> isize;

// socket
pub type SmoltcpAddrInfoFunc = fn(&str) -> Result<core::net::Ipv4Addr, ()>;
pub type SmoltcpConnectFunc = fn(SocketAddrV4) -> Result<(), ()>;
pub type SmoltcpSendFunc = fn(&[u8]) -> Result<(), ()>;
pub type SmoltcpRecvFunc = fn(&mut [u8]) -> Result<usize, ()>;
pub type InitDevFunc = fn(NetdevName);
pub type NetdevAllocFunc = fn() -> Result<NetdevName, ()>;

// buffer_alloc
pub type BufferAllocFunc = fn(Layout) -> Result<usize, ()>;
pub type AccessBufferFunc = fn() -> Option<usize>;

pub trait Transmutor {
    fn find_host_call() -> FindHostCallFunc;
    fn host_panic_handler() -> PanicHandlerFunc;
}

pub macro func_type {
    (write) => (ms_hostcall::types::HostWriteFunc),
    (stdout) => (ms_hostcall::types::HostStdioFunc),
    (addrinfo) => (ms_hostcall::types::SmoltcpAddrInfoFunc),
    (connect) => (ms_hostcall::types::SmoltcpConnectFunc),
    (send) => (ms_hostcall::types::SmoltcpSendFunc),
    (recv) => (ms_hostcall::types::SmoltcpRecvFunc),
    (buffer_alloc) => (ms_hostcall::types::BufferAllocFunc),
    (access_buffer) => (ms_hostcall::types::AccessBufferFunc)
}
