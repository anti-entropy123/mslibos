use std::sync::Mutex;

use lazy_static::lazy_static;
use ms_hostcall::{
    types::{
        FindHostCallFunc, HostCallError as HCError, HostCallResult as HCResult, HostWriteFunc,
    },
    CommonHostCall, HostCallID, Transmutor,
};

pub struct UserHostCall {
    pub msvisor_handler_addr: Option<usize>,
    pub write_addr: Option<usize>,
}

impl UserHostCall {
    fn new() -> Self {
        UserHostCall {
            write_addr: None,
            msvisor_handler_addr: None,
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
        unsafe { core::mem::transmute(self.msvisor_handler_addr.unwrap()) }
    }
}

impl UserHostCall {
    pub fn host_write(fd: i32, buf: &str) -> isize {
        let write: fn(i32, &str) -> isize = {
            let mut hostcall_table = USER_HOST_CALL.lock().unwrap();
            if hostcall_table.write_addr.is_none() {
                let find_host_call: FindHostCallFunc = hostcall_table.find_host_call();
                let write_addr =
                    unsafe { find_host_call(HostCallID::Common(CommonHostCall::Write)) };
                hostcall_table.write_addr = Some(write_addr);
            };
            hostcall_table.host_write_func()
        };

        write(fd, buf)
    }
}

#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub extern "C" fn set_handler_addr(addr: usize) -> HCResult {
    let mut hostcalls = USER_HOST_CALL.lock().unwrap();
    if hostcalls.write_addr.is_some() {
        return Err(HCError::HasBeenSet);
    };
    hostcalls.write_addr = Some(addr);
    Ok(())
}

#[no_mangle]
pub extern "C" fn get_handler_addr() -> usize {
    USER_HOST_CALL.lock().unwrap().write_addr.unwrap()
}
