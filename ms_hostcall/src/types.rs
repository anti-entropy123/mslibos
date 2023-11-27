#![allow(improper_ctypes_definitions)]

use core::ffi::c_void;
use core::{alloc::Layout, net::SocketAddrV4};

use alloc::{collections::BTreeMap, string::String};
// use smoltcp::{iface::Interface, phy::TunTapInterface};

use crate::{err::LibOSResult, HostCallID, IsolationContext};

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
pub type SetHandlerFunc = unsafe extern "C" fn(&IsolationContext) -> HostCallResult;
pub type GetHandlerFunc = unsafe extern "C" fn() -> usize;
pub type PanicHandlerFunc = unsafe extern "C" fn() -> !;

// service drop
pub type DropHandlerFunc = unsafe fn();

// app main
pub type RustMainFunc = unsafe fn(&BTreeMap<String, String>) -> Result<(), ()>;

// fdtab
bitflags! {
    pub struct OpenFlags: u32 {
       const O_APPEND = 1;
       const O_CREAT = 2;
    }

    #[derive(PartialEq, Clone)]
    pub struct OpenMode: u32 {
        const RD = 1;
        const WR = 2;
        const RDWR = 3;
    }
}
pub type Fd = u32;
pub type Size = usize;

pub type OpenFunc = fn(&str, OpenFlags, OpenMode) -> Result<Fd, ()>;
pub type WriteFunc = fn(Fd, &[u8]) -> LibOSResult<Size>;
pub type ReadFunc = fn(Fd, &mut [u8]) -> LibOSResult<Size>;
pub type CloseFunc = fn(Fd) -> Result<(), ()>;
pub type LseekFunc = fn(Fd, u32) -> LibOSResult<()>;
pub struct Stat {
    pub st_size: Size,
}
pub type StatFunc = fn(Fd) -> Result<Stat, ()>;
pub type ConnectFunc = fn(SocketAddrV4) -> Result<Fd, ()>;
pub type BindFunc = fn(SocketAddrV4) -> LibOSResult<Fd>;
pub type AcceptFunc = fn(SockFd) -> LibOSResult<SockFd>;

// stdio
pub type HostStdioFunc = fn(&[u8]) -> Size;

// socket
pub type SockFd = u32;

// time
pub type GetTimeFunc = fn() -> Result<u128, ()>;

// mm
pub type BufferAllocFunc = fn(&str, Layout, u64) -> Result<usize, ()>;
pub type AccessBufferFunc = fn(&str) -> Option<(usize, u64)>;
pub type BufferDeallocFunc = fn(usize, Layout);
bitflags! {
    #[derive(PartialEq, Eq)]
    pub struct ProtFlags: u32 {
        const READ = 1;
        const WRITE = 2;
        const EXEC = 3;
    }
}
pub type MemmapFunc = fn(usize, ProtFlags, Fd) -> LibOSResult<usize>;
pub type MemunmapFunc = fn(&mut [u8]) -> LibOSResult<()>;

// mmap_file_backend
pub type RegisterFileBackendFunc = fn(&mut [c_void], Fd) -> LibOSResult<()>;
pub type UnregisterFileBackendFunc = fn(usize) -> LibOSResult<()>;
pub type FilePageFaultHandlerFunc = fn();

// isol_info
pub type MetricFunc = fn(IsolationID, MetricEvent) -> Result<(), ()>;
pub type FsImageFunc = fn(IsolationID) -> Option<String>;
pub type SpawnFaultThreadFunc = fn(IsolationID) -> Result<(), String>;

#[derive(Debug)]
pub enum MetricEvent {
    // IsolationEvent
    IsolBegin,
    LoadService,
    IsolEnd,

    // SvcEvent
    SvcInit,
    SvcRun,
    SvcEnd,

    // MesureEvent
    Mem,
}

pub trait Transmutor {
    fn find_host_call() -> FindHostCallFunc;
    fn host_panic_handler() -> PanicHandlerFunc;
}
