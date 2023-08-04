use lazy_static::lazy_static;
use ms_hostcall::{
    types::{FindHostCallFunc, PanicHandlerFunc, Transmutor},
    CommonHostCall, HostCallID,
};

use crate::init_context::isolation_ctx;
use crate::sync::UPSafeCell;

pub struct UserHostCall {
    write_addr: Option<usize>,
    stdout_addr: Option<usize>,
    smoltcp_addrinfo_addr: Option<usize>,
    smoltcp_connect: Option<usize>,
    smoltcp_send: Option<usize>,
    smoltcp_recv: Option<usize>,
    alloc_buffer: Option<usize>,
    access_buffer: Option<usize>,
}

impl UserHostCall {
    fn new() -> Self {
        UserHostCall {
            write_addr: None,
            stdout_addr: None,
            smoltcp_addrinfo_addr: None,
            smoltcp_connect: None,
            smoltcp_send: None,
            smoltcp_recv: None,
            alloc_buffer: None,
            access_buffer: None,
        }
    }
}

lazy_static! {
    pub static ref USER_HOST_CALL: UPSafeCell<UserHostCall> =
        unsafe { UPSafeCell::new(UserHostCall::new()) };
}

impl Transmutor for UserHostCall {
    fn find_host_call() -> FindHostCallFunc {
        unsafe { core::mem::transmute(isolation_ctx().find_handler) }
    }

    fn host_panic_handler() -> PanicHandlerFunc {
        unsafe { core::mem::transmute(isolation_ctx().panic_handler) }
    }
}

impl UserHostCall {
    pub fn get_or_find(&mut self, chc_id: CommonHostCall) -> usize {
        let entry_addr = match chc_id {
            CommonHostCall::Write => &mut self.write_addr,
            CommonHostCall::Stdout => &mut self.stdout_addr,
            CommonHostCall::SmoltcpAddrInfo => &mut self.smoltcp_addrinfo_addr,
            CommonHostCall::SmoltcpConnect => &mut self.smoltcp_connect,
            CommonHostCall::SmoltcpSend => &mut self.smoltcp_send,
            CommonHostCall::SmoltcpRecv => &mut self.smoltcp_recv,
            CommonHostCall::BufferAlloc => &mut self.alloc_buffer,
            CommonHostCall::AccessBuffer => &mut self.access_buffer,
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

pub macro libos {
    ($name:ident($($arg_name:expr),*)) => {
        {
            fn binding() -> ms_hostcall::types::func_type!($name) {
                let mut table = USER_HOST_CALL.exclusive_access();
                unsafe { core::mem::transmute(table.get_or_find(ms_hostcall::hostcall_id!($name))) }
            }
            let $name = binding();
            $name($($arg_name),*)
        }
    }
}
