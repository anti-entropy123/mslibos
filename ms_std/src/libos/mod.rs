use lazy_static::lazy_static;
use ms_hostcall::{
    types::{FindHostCallFunc, PanicHandlerFunc, Transmutor},
    CommonHostCall, HostCallID,
};

use crate::init_context::isolation_ctx;
use crate::sync::UPSafeCell;

mod utils;
pub use utils::libos;

pub struct UserHostCall {
    write_addr: Option<usize>,
    open_addr: Option<usize>,
    read_addr: Option<usize>,
    close_addr: Option<usize>,
    connect_addr: Option<usize>,

    stdout_addr: Option<usize>,

    fatfs_open_addr: Option<usize>,
    fatfs_write_addr: Option<usize>,
    fatfs_read_addr: Option<usize>,
    fatfs_close_addr: Option<usize>,

    smoltcp_addrinfo_addr: Option<usize>,
    smoltcp_connect: Option<usize>,
    smoltcp_send: Option<usize>,
    smoltcp_recv: Option<usize>,

    alloc_buffer: Option<usize>,
    access_buffer: Option<usize>,

    get_time: Option<usize>,
}

impl UserHostCall {
    fn new() -> Self {
        UserHostCall {
            write_addr: None,
            open_addr: None,
            read_addr: None,
            close_addr: None,
            connect_addr: None,

            stdout_addr: None,

            fatfs_open_addr: None,
            fatfs_write_addr: None,
            fatfs_read_addr: None,
            fatfs_close_addr: None,

            smoltcp_addrinfo_addr: None,
            smoltcp_connect: None,
            smoltcp_send: None,
            smoltcp_recv: None,

            alloc_buffer: None,
            access_buffer: None,

            get_time: None,
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
            CommonHostCall::Open => &mut self.open_addr,
            CommonHostCall::Read => &mut self.read_addr,
            CommonHostCall::Close => &mut self.close_addr,
            CommonHostCall::Connect => &mut self.connect_addr,

            CommonHostCall::Stdout => &mut self.stdout_addr,

            CommonHostCall::FatfsOpen => &mut self.fatfs_open_addr,
            CommonHostCall::FatfsWrite => &mut self.fatfs_write_addr,
            CommonHostCall::FatfsRead => &mut self.fatfs_read_addr,
            CommonHostCall::FatfsClose => &mut self.fatfs_close_addr,

            CommonHostCall::SmoltcpAddrInfo => &mut self.smoltcp_addrinfo_addr,
            CommonHostCall::SmoltcpConnect => &mut self.smoltcp_connect,
            CommonHostCall::SmoltcpSend => &mut self.smoltcp_send,
            CommonHostCall::SmoltcpRecv => &mut self.smoltcp_recv,

            CommonHostCall::BufferAlloc => &mut self.alloc_buffer,
            CommonHostCall::AccessBuffer => &mut self.access_buffer,

            CommonHostCall::GetTime => &mut self.get_time,
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
