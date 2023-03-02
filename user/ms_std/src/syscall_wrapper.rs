use std::sync::Mutex;

use lazy_static::lazy_static;
use ms_hostcall::{FindHostCall, HostCall, HostCallID};
use once_cell::sync::OnceCell;

static MSVISOR_HANDLER_ADDR: OnceCell<usize> = OnceCell::new();

pub struct UserHostCall {
    pub write_addr: Option<usize>,
}

impl UserHostCall {
    fn new() -> Self {
        UserHostCall { write_addr: None }
    }
}

lazy_static! {
    pub static ref USER_HOST_CALL: Mutex<UserHostCall> = Mutex::new(UserHostCall::new());
}

impl HostCall for UserHostCall {
    fn host_write(fd: i32, buf: &str) -> isize {
        let write: fn(i32, &str) -> isize = {
            let mut hostcall_table = USER_HOST_CALL.lock().unwrap();
            if hostcall_table.write_addr.is_none() {
                let find_host_call: FindHostCall = get_msvisor_handler();
                hostcall_table.write_addr = Some(find_host_call(HostCallID::Write));
            };
            unsafe { core::mem::transmute(hostcall_table.write_addr.unwrap()) }
        };

        write(fd, buf)
    }
}

#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub extern "C" fn set_handler_addr(addr: usize) -> Result<(), ()> {
    let handler = *MSVISOR_HANDLER_ADDR.get_or_init(|| addr);
    // println!("handler=0x{:x}, addr=0x{:x}", handler, addr);
    assert!(addr == handler, "duplicate set handler addr");
    Ok(())
}

#[no_mangle]
pub extern "C" fn get_handler_addr() -> usize {
    *MSVISOR_HANDLER_ADDR.get().unwrap()
}

fn get_msvisor_handler() -> FindHostCall {
    unsafe { core::mem::transmute(get_handler_addr()) }
}
