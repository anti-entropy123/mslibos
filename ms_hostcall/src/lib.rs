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
    #[display(fmt = "host_write")]
    Write,
    #[display(fmt = "host_stdout")]
    Stdout,
    // #[display(fmt = "init_net_dev")]
    // SmoltcpInitDev,
    #[display(fmt = "addrinfo")]
    SmoltcpAddrInfo,
    #[display(fmt = "connect")]
    SmoltcpConnect,
    #[display(fmt = "send")]
    SmoltcpSend,
    #[display(fmt = "recv")]
    SmoltcpRecv,
    // #[display(fmt = "netdev_alloc")]
    // NetdevAlloc,
    // #[display(fmt = "netdev_dealloc")]
    // NetdevDealloc,
    #[display(fmt = "buffer_alloc")]
    BufferAlloc,
    #[display(fmt = "access_buffer")]
    AccessBuffer,
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
                CommonHostCall::Write => "fdtab".to_owned(),

                CommonHostCall::Stdout => "stdio".to_owned(),
                
                CommonHostCall::SmoltcpAddrInfo => "socket".to_owned(),
                CommonHostCall::SmoltcpConnect => "socket".to_owned(),
                CommonHostCall::SmoltcpSend => "socket".to_owned(),
                CommonHostCall::SmoltcpRecv => "socket".to_owned(),
                
                CommonHostCall::BufferAlloc => "buffer".to_owned(),
                CommonHostCall::AccessBuffer => "buffer".to_owned(),
            },
            HostCallID::Custom(_) => todo!(),
        }
    }
}

pub macro hostcall_id {
    (write) => (ms_hostcall::CommonHostCall::Write),
    (stdout) => (ms_hostcall::CommonHostCall::Stdout),
    (addrinfo) => (ms_hostcall::CommonHostCall::SmoltcpAddrInfo),
    (connect) => (ms_hostcall::CommonHostCall::SmoltcpConnect),
    (send) => (ms_hostcall::CommonHostCall::SmoltcpSend),
    (recv) => (ms_hostcall::CommonHostCall::SmoltcpRecv),
    (buffer_alloc) => (ms_hostcall::CommonHostCall::BufferAlloc),
    (access_buffer) => (ms_hostcall::CommonHostCall::AccessBuffer)
}

#[test]
fn format_hostcall_id() {
    use crate::alloc::string::ToString;

    let result = CommonHostCall::Write;
    assert!(
        result.to_string().eq("host_write"),
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

pub const SERVICE_HEAP_SIZE: usize = 4096 * 8;
