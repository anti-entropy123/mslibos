use lazy_static::lazy_static;
use ms_hostcall::{
    types::{FindHostCallFunc, HostWriteFunc},
    CommonHostCall, HostCallID, IsolationContext, Transmutor,
};

use crate::sync::UPSafeCell;

pub struct UserHostCall {
    pub isolation_ctx: Option<IsolationContext>,
    pub write_addr: Option<usize>,
    pub stdout_addr: Option<usize>,
}

impl UserHostCall {
    fn new() -> Self {
        UserHostCall {
            write_addr: None,
            isolation_ctx: None,
            stdout_addr: None,
        }
    }
}

lazy_static! {
    pub static ref USER_HOST_CALL: UPSafeCell<UserHostCall> =
        unsafe { UPSafeCell::new(UserHostCall::new()) };
}

impl Transmutor for UserHostCall {
    fn host_write_func(&self) -> HostWriteFunc {
        unsafe { core::mem::transmute(self.write_addr.unwrap()) }
    }

    fn find_host_call(&self) -> FindHostCallFunc {
        unsafe { core::mem::transmute(self.isolation_ctx.unwrap().find_handler) }
    }
}

impl UserHostCall {
    pub fn host_write(fd: i32, buf: &str) -> isize {
        let write: fn(i32, &str) -> isize = {
            let mut hostcall_table = USER_HOST_CALL.exclusive_access();
            if hostcall_table.write_addr.is_none() {
                let find_host_call: FindHostCallFunc = hostcall_table.find_host_call();
                let write_addr = unsafe {
                    find_host_call(
                        hostcall_table.isolation_ctx.unwrap().isol_id,
                        HostCallID::Common(CommonHostCall::Write),
                    )
                };
                hostcall_table.write_addr = Some(write_addr);
            };
            hostcall_table.host_write_func()
        };

        write(fd, buf)
    }
}
