#![allow(improper_ctypes_definitions)]

use alloc::{collections::BTreeMap, string::String};

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
pub type SetHandlerFunc = unsafe extern "C" fn(&IsolationContext) -> HostCallResult;
pub type GetHandlerFunc = unsafe extern "C" fn() -> usize;
pub type PanicHandlerFunc = unsafe extern "C" fn() -> !;

// service drop
pub type DropHandlerFunc = unsafe fn();

// app main
pub type RustMainFunc = unsafe fn(&BTreeMap<String, String>)/* -> Result<(), String>*/;

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
pub struct Stat {
    pub st_size: Size,
}

// stdio
pub type HostStdioFunc = fn(&[u8]) -> Size;

// socket
pub type SockFd = u32;

// time
pub type GetTimeFunc = fn() -> Result<u128, String>;
pub type NanoSleepFunc = fn(u64, u64);

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
