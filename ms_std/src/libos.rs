use core::net::{Ipv4Addr, SocketAddrV4};

use lazy_static::lazy_static;
use ms_hostcall::{
    types::{
        FindHostCallFunc, HostStdioFunc, HostWriteFunc, PanicHandlerFunc, SmoltcpAddrInfoFunc,
        SmoltcpConnectFunc, SmoltcpRecvFunc, SmoltcpSendFunc, Transmutor,
    },
    CommonHostCall, HostCallID,
};

use crate::init_context::isolation_ctx;
use crate::sync::UPSafeCell;

pub struct UserHostCall {
    write_addr: Option<usize>,
    stdout_addr: Option<usize>,
    smoltcp_init_dev: Option<usize>,
    smoltcp_addrinfo_addr: Option<usize>,
    smoltcp_connect: Option<usize>,
    smoltcp_send: Option<usize>,
    smoltcp_recv: Option<usize>,
}

impl UserHostCall {
    fn new() -> Self {
        UserHostCall {
            write_addr: None,
            stdout_addr: None,
            smoltcp_init_dev: None,
            smoltcp_addrinfo_addr: None,
            smoltcp_connect: None,
            smoltcp_send: None,
            smoltcp_recv: None,
        }
    }
}

lazy_static! {
    pub static ref USER_HOST_CALL: UPSafeCell<UserHostCall> =
        unsafe { UPSafeCell::new(UserHostCall::new()) };
}

/// This impl would refactor with procedure macro.
impl Transmutor for UserHostCall {
    fn find_host_call() -> FindHostCallFunc {
        unsafe { core::mem::transmute(isolation_ctx().find_handler) }
    }

    fn host_panic_handler() -> PanicHandlerFunc {
        unsafe { core::mem::transmute(isolation_ctx().panic_handler) }
    }

    fn host_write_func(&mut self) -> HostWriteFunc {
        unsafe { core::mem::transmute(self.get_or_find(CommonHostCall::Write)) }
    }

    fn host_stdio_func(&mut self) -> HostStdioFunc {
        unsafe { core::mem::transmute(self.get_or_find(CommonHostCall::Stdout)) }
    }

    fn smoltcp_addrinfo(&mut self) -> SmoltcpAddrInfoFunc {
        unsafe { core::mem::transmute(self.get_or_find(CommonHostCall::SmoltcpAddrInfo)) }
    }

    fn smoltcp_connect(&mut self) -> SmoltcpConnectFunc {
        unsafe { core::mem::transmute(self.get_or_find(CommonHostCall::SmoltcpConnect)) }
    }

    fn smoltcp_send(&mut self) -> ms_hostcall::types::SmoltcpSendFunc {
        unsafe { core::mem::transmute(self.get_or_find(CommonHostCall::SmoltcpSend)) }
    }

    fn smoltcp_recv(&mut self) -> ms_hostcall::types::SmoltcpRecvFunc {
        unsafe { core::mem::transmute(self.get_or_find(CommonHostCall::SmoltcpRecv)) }
    }
}

impl UserHostCall {
    fn get_or_find(&mut self, chc_id: CommonHostCall) -> usize {
        let entry_addr = match chc_id {
            CommonHostCall::Write => &mut self.write_addr,
            CommonHostCall::Stdout => &mut self.stdout_addr,
            CommonHostCall::SmoltcpAddrInfo => &mut self.smoltcp_addrinfo_addr,
            CommonHostCall::SmoltcpInitDev => &mut self.smoltcp_init_dev,
            CommonHostCall::SmoltcpConnect => &mut self.smoltcp_connect,
            CommonHostCall::SmoltcpSend => &mut self.smoltcp_send,
            CommonHostCall::SmoltcpRecv => &mut self.smoltcp_recv,
        };
        if entry_addr.is_none() {
            let find_host_call = UserHostCall::find_host_call();
            let addr =
                unsafe { find_host_call(isolation_ctx().isol_id, HostCallID::Common(chc_id)) };
            *entry_addr = Some(addr);
            addr
        } else {
            entry_addr.unwrap()
        }
    }
}

pub fn host_write(fd: i32, buf: &str) -> isize {
    let write: HostWriteFunc = {
        let mut hostcall_table = USER_HOST_CALL.exclusive_access();
        hostcall_table.host_write_func()
    };

    write(fd, buf)
}

pub fn stdout(buf: &str) -> isize {
    let stdout: HostStdioFunc = {
        let mut hostcall_table = USER_HOST_CALL.exclusive_access();
        hostcall_table.host_stdio_func()
    };

    stdout(buf)
}

pub fn addr_info(name: &str) -> Result<Ipv4Addr, ()> {
    let addr_info: SmoltcpAddrInfoFunc = {
        let mut hostcall_table = USER_HOST_CALL.exclusive_access();
        hostcall_table.smoltcp_addrinfo()
    };

    addr_info(name)
}

pub fn connect(sockaddr: SocketAddrV4) -> Result<(), ()> {
    let connect: SmoltcpConnectFunc = {
        let mut hostcall_table = USER_HOST_CALL.exclusive_access();
        hostcall_table.smoltcp_connect()
    };
    connect(sockaddr)
}

pub fn send(data: &[u8]) -> Result<(), ()> {
    let send: SmoltcpSendFunc = {
        let mut hostcall_table = USER_HOST_CALL.exclusive_access();
        hostcall_table.smoltcp_send()
    };

    send(data)
}

pub fn recv(buf: &mut [u8]) -> Result<usize, ()> {
    let recv: SmoltcpRecvFunc = {
        let mut hostcall_table = USER_HOST_CALL.exclusive_access();
        hostcall_table.smoltcp_recv()
    };

    recv(buf)
}
