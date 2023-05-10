use core::net::SocketAddrV4;

use alloc::string::String;
// use smoltcp::{iface::Interface, phy::TunTapInterface};

use crate::{HostCallID, IsolationContext};

pub type IsolationID = u64;
pub type ServiceName = String;
pub type SymbolName = String;
pub type HostCallResult = Result<(), HostCallError>;
// pub type NetDevice = TunTapInterface;
// pub type NetIface = Interface;

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

pub trait Transmutor {
    fn find_host_call() -> FindHostCallFunc;
    fn host_panic_handler() -> PanicHandlerFunc;

    fn host_write_func(&mut self) -> HostWriteFunc;
    fn host_stdio_func(&mut self) -> HostStdioFunc;
    fn smoltcp_addrinfo(&mut self) -> SmoltcpAddrInfoFunc;
    fn smoltcp_connect(&mut self) -> SmoltcpConnectFunc;
    fn smoltcp_send(&mut self) -> SmoltcpSendFunc;
    fn smoltcp_recv(&mut self) -> SmoltcpRecvFunc;
}
