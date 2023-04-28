use core::net::Ipv4Addr;

use lazy_static::lazy_static;
use ms_hostcall::{
    types::{
        FindHostCallFunc, HostStdioFunc, HostWriteFunc, PanicHandlerFunc, SomltcpAddrInfoFunc,
        Transmutor,
    },
    CommonHostCall, HostCallID,
};

use crate::{init_context::isolation_ctx, sync::UPSafeCell};

pub struct UserHostCall {
    write_addr: Option<usize>,
    stdout_addr: Option<usize>,
    smoltcp_addrinfo_addr: Option<usize>,
}

impl UserHostCall {
    fn new() -> Self {
        UserHostCall {
            write_addr: None,
            stdout_addr: None,
            smoltcp_addrinfo_addr: None,
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

    fn somltcp_addrinfo(&mut self) -> SomltcpAddrInfoFunc {
        unsafe { core::mem::transmute(self.get_or_find(CommonHostCall::SmoltcpAddrInfo)) }
    }
}

impl UserHostCall {
    fn get_or_find(&mut self, chc_id: CommonHostCall) -> usize {
        let entry_addr = match chc_id {
            CommonHostCall::Write => &mut self.write_addr,
            CommonHostCall::Stdout => &mut self.stdout_addr,
            CommonHostCall::SmoltcpAddrInfo => &mut self.smoltcp_addrinfo_addr,
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
    let addr_info: SomltcpAddrInfoFunc = {
        let mut hostcall_table = USER_HOST_CALL.exclusive_access();
        hostcall_table.somltcp_addrinfo()
    };

    addr_info(name)
}
