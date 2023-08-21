#![allow(improper_ctypes_definitions)]

use core::{alloc::Layout, net::SocketAddrV4};

use alloc::string::String;
// use smoltcp::{iface::Interface, phy::TunTapInterface};

use crate::{HostCallID, IsolationContext};

use bitflags::bitflags;

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
bitflags! {
    pub struct OpenFlags: u32 {
       const O_APPEND = 1;
       const O_CREAT = 2;
    }
}
pub type Fd = u32;
pub type Size = usize;

#[derive(PartialEq, Clone)]
pub enum OpenMode {
    RDONLY,
    WRONLY,
    RDWR,
}
pub type HostOpenFunc = fn(&str, OpenFlags, OpenMode) -> Result<Fd, ()>;
pub type HostWriteFunc = fn(Fd, &[u8]) -> Result<Size, ()>;
pub type HostReadFunc = fn(Fd, &mut [u8]) -> Result<Size, ()>;
pub type HostCloseFunc = fn(Fd) -> Result<(), ()>;

// stdio
pub type HostStdioFunc = fn(&[u8]) -> Size;

// Fatfs
pub type FatfsOpenFunc = fn(&str, OpenFlags) -> Result<Fd, ()>;
pub type FatfsWriteFunc = fn(Fd, &[u8]) -> Result<Size, ()>;
pub type FatfsReadFunc = fn(Fd, &mut [u8]) -> Result<Size, ()>;
pub type FatfsCloseFunc = fn(Fd) -> Result<(), ()>;

// socket
pub type SmoltcpAddrInfoFunc = fn(&str) -> Result<core::net::Ipv4Addr, ()>;
pub type SmoltcpConnectFunc = fn(SocketAddrV4) -> Result<(), ()>;
pub type SmoltcpSendFunc = fn(&[u8]) -> Result<(), ()>;
pub type SmoltcpRecvFunc = fn(&mut [u8]) -> Result<Size, ()>;
pub type InitDevFunc = fn(NetdevName);
pub type NetdevAllocFunc = fn() -> Result<NetdevName, ()>;

// time
pub type GetTimeFunc = fn() -> Result<u128, ()>;

// buffer_alloc
pub type BufferAllocFunc = fn(Layout, u64) -> Result<usize, ()>;
pub type AccessBufferFunc = fn() -> Option<(usize, u64)>;

pub trait Transmutor {
    fn find_host_call() -> FindHostCallFunc;
    fn host_panic_handler() -> PanicHandlerFunc;
}
