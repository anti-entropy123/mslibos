use std::sync::Mutex;

use lazy_static::lazy_static;
use ms_hostcall::{
    types::{
        FindHostCallFunc, HostCallResult as HCResult, HostWriteFunc,
    },
    CommonHostCall, HostCallID, IsolationContext, Transmutor,
};

pub struct UserHostCall {
    pub isolation_ctx: Option<IsolationContext>,
    pub write_addr: Option<usize>,
}

impl UserHostCall {
    fn new() -> Self {
        UserHostCall {
            write_addr: None,
            isolation_ctx: None,
        }
    }
}

lazy_static! {
    pub static ref USER_HOST_CALL: Mutex<UserHostCall> = Mutex::new(UserHostCall::new());
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
            let mut hostcall_table = USER_HOST_CALL.lock().unwrap();
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

#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub extern "C" fn set_handler_addr(ctx: IsolationContext) -> HCResult {
    let mut hostcalls = USER_HOST_CALL.lock().unwrap();
    if hostcalls.isolation_ctx.is_some() {
        panic!();
        // return Err(HCError::HasBeenSet);
    };
    hostcalls.isolation_ctx = Some(ctx);
    Ok(())
}

#[no_mangle]
pub extern "C" fn get_handler_addr() -> usize {
    USER_HOST_CALL
        .lock()
        .unwrap()
        .isolation_ctx
        .unwrap()
        .find_handler
}
