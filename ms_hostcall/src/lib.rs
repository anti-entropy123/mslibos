#![no_std]
#![feature(ip_in_core)]

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
    #[display(fmt = "init_net_dev")]
    SmoltcpInitDev,
    #[display(fmt = "addrinfo")]
    SmoltcpAddrInfo,
    #[display(fmt = "connect")]
    SmoltcpConnect,
    #[display(fmt = "send")]
    SmoltcpSend,
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
                CommonHostCall::SmoltcpInitDev => "socket".to_owned(),
                CommonHostCall::SmoltcpConnect => "socket".to_owned(),
                CommonHostCall::SmoltcpSend => "socket".to_owned(),
            },
            HostCallID::Custom(_) => todo!(),
        }
    }
}

#[test]
fn format_hostcall_id() {
    let result = HostCallID::Common(CommonHostCall::Write).to_string();
    assert!(
        result.eq("host_write"),
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
