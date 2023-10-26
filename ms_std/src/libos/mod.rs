use lazy_static::lazy_static;
use spin::Mutex;

use crate::init_context::isolation_ctx;
pub use ms_hostcall::types::MetricEvent;
use ms_hostcall::{
    types::{FindHostCallFunc, PanicHandlerFunc, Transmutor},
    CommonHostCall, HostCallID,
};
mod utils;
pub use utils::libos;

lazy_static! {
    pub static ref USER_HOST_CALL: Mutex<UserHostCall> = Mutex::new(UserHostCall::new());
}

#[derive(Default)]
pub struct UserHostCall {
    write_addr: Option<usize>,
    open_addr: Option<usize>,
    read_addr: Option<usize>,
    close_addr: Option<usize>,
    connect_addr: Option<usize>,
    socket_addr: Option<usize>,
    bind_addr: Option<usize>,
    accept_addr: Option<usize>,

    stdout_addr: Option<usize>,

    fatfs_open_addr: Option<usize>,
    fatfs_write_addr: Option<usize>,
    fatfs_read_addr: Option<usize>,
    fatfs_close_addr: Option<usize>,

    smoltcp_addrinfo_addr: Option<usize>,
    smoltcp_connect_addr: Option<usize>,
    smoltcp_send_addr: Option<usize>,
    smoltcp_recv_addr: Option<usize>,
    smoltcp_bind_addr: Option<usize>,
    smoltcp_accept_addr: Option<usize>,
    smoltcp_close_addr: Option<usize>,

    alloc_buffer_addr: Option<usize>,
    access_buffer_addr: Option<usize>,

    get_time_addr: Option<usize>,

    metric_addr: Option<usize>,
}

impl UserHostCall {
    fn new() -> Self {
        UserHostCall::default()
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
            CommonHostCall::Socket => &mut self.socket_addr,
            CommonHostCall::Bind => &mut self.bind_addr,
            CommonHostCall::Accept => &mut self.accept_addr,

            CommonHostCall::Stdout => &mut self.stdout_addr,

            CommonHostCall::FatfsOpen => &mut self.fatfs_open_addr,
            CommonHostCall::FatfsWrite => &mut self.fatfs_write_addr,
            CommonHostCall::FatfsRead => &mut self.fatfs_read_addr,
            CommonHostCall::FatfsClose => &mut self.fatfs_close_addr,

            CommonHostCall::SmoltcpAddrInfo => &mut self.smoltcp_addrinfo_addr,
            CommonHostCall::SmoltcpConnect => &mut self.smoltcp_connect_addr,
            CommonHostCall::SmoltcpSend => &mut self.smoltcp_send_addr,
            CommonHostCall::SmoltcpRecv => &mut self.smoltcp_recv_addr,
            CommonHostCall::SmoltcpBind => &mut self.smoltcp_bind_addr,
            CommonHostCall::SmoltcpAccept => &mut self.smoltcp_accept_addr,
            CommonHostCall::SmoltcpClose => &mut self.smoltcp_close_addr,

            CommonHostCall::BufferAlloc => &mut self.alloc_buffer_addr,
            CommonHostCall::AccessBuffer => &mut self.access_buffer_addr,

            CommonHostCall::GetTime => &mut self.get_time_addr,
            CommonHostCall::Metric => &mut self.metric_addr,
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

impl Transmutor for UserHostCall {
    fn find_host_call() -> FindHostCallFunc {
        unsafe { core::mem::transmute(isolation_ctx().find_handler) }
    }

    fn host_panic_handler() -> PanicHandlerFunc {
        unsafe { core::mem::transmute(isolation_ctx().panic_handler) }
    }
}

pub fn metric(event: MetricEvent) {
    if !matches!(event, MetricEvent::Mem) {
        panic!("unallowed event")
    };

    if libos!(metric(isolation_ctx().isol_id, event)).is_err() {}
}
