#![no_std]
#![feature(ip_in_core)]
#![feature(decl_macro)]

extern crate alloc;

pub mod types;
use alloc::{borrow::ToOwned, string::String};
use types::{IsolationID, ServiceName};

use derive_more::Display;

#[derive(Debug, Display)]
#[repr(C)]
pub enum CommonHostCall {
    #[display(fmt = "write")]
    Write,
    #[display(fmt = "read")]
    Read,
    #[display(fmt = "open")]
    Open,
    #[display(fmt = "close")]
    Close,
    #[display(fmt = "connect")]
    Connect,

    #[display(fmt = "host_stdout")]
    Stdout,

    #[display(fmt = "fatfs_open")]
    FatfsOpen,
    #[display(fmt = "fatfs_write")]
    FatfsWrite,
    #[display(fmt = "fatfs_read")]
    FatfsRead,
    #[display(fmt = "fatfs_close")]
    FatfsClose,

    #[display(fmt = "addrinfo")]
    SmoltcpAddrInfo,
    #[display(fmt = "smol_connect")]
    SmoltcpConnect,
    #[display(fmt = "smol_send")]
    SmoltcpSend,
    #[display(fmt = "smol_recv")]
    SmoltcpRecv,

    // #[display(fmt = "netdev_alloc")]
    // NetdevAlloc,
    // #[display(fmt = "netdev_dealloc")]
    // NetdevDealloc,
    #[display(fmt = "buffer_alloc")]
    BufferAlloc,
    #[display(fmt = "access_buffer")]
    AccessBuffer,

    #[display(fmt = "get_time")]
    GetTime,
}

#[derive(Debug, Display)]
#[repr(C)]
pub enum HostCallID {
    Common(CommonHostCall),
    Custom(String),
}
impl HostCallID {
    pub fn belong_to(&self) -> ServiceName {
        match self {
            Self::Common(common) => match common {
                CommonHostCall::Write
                | CommonHostCall::Open
                | CommonHostCall::Read
                | CommonHostCall::Close
                | CommonHostCall::Connect => "fdtab".to_owned(),

                CommonHostCall::Stdout => "stdio".to_owned(),

                CommonHostCall::FatfsOpen
                | CommonHostCall::FatfsWrite
                | CommonHostCall::FatfsRead
                | CommonHostCall::FatfsClose => "fatfs".to_owned(),

                CommonHostCall::SmoltcpAddrInfo
                | CommonHostCall::SmoltcpConnect
                | CommonHostCall::SmoltcpSend
                | CommonHostCall::SmoltcpRecv => "socket".to_owned(),

                CommonHostCall::BufferAlloc | CommonHostCall::AccessBuffer => "buffer".to_owned(),

                CommonHostCall::GetTime => "time".to_owned(),
            },
            HostCallID::Custom(_) => todo!(),
        }
    }
}

#[test]
fn format_hostcall_id() {
    use crate::alloc::string::ToString;

    let result = CommonHostCall::Write;
    assert!(
        result.to_string().eq("write"),
        "actual format result is {}",
        result
    )
}

#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct IsolationContext {
    pub isol_id: IsolationID,
    pub find_handler: usize,
    pub panic_handler: usize,
    pub heap_range: (usize, usize),
}

pub const SERVICE_HEAP_SIZE: usize = 4096 * 16;

pub trait Verify {
    fn __fingerprint() -> u64;
}
