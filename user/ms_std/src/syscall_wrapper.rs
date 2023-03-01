use ms_hostcall::{FindHostCall, HostCall, HostCallID};
use once_cell::sync::OnceCell;

static MSVISOR_HANDLER_ADDR: OnceCell<usize> = OnceCell::new();

pub struct UserHostCall {
    write_addr: Option<usize>,
}

pub static mut USER_HOST_CALL: UserHostCall = UserHostCall { write_addr: None };

impl HostCall for UserHostCall {
    fn host_write(fd: i32, buf: &str) -> isize {
        if let None = unsafe { USER_HOST_CALL.write_addr } {
            let handler_addr = *MSVISOR_HANDLER_ADDR.get().unwrap();
            // let find_host_call = unsafe { *handler };
            let find_host_call: FindHostCall = unsafe { core::mem::transmute(handler_addr) };
            unsafe { USER_HOST_CALL.write_addr = Some(find_host_call(HostCallID::Write)) };
        };
        let write: fn(i32, &str) -> isize =
            unsafe { core::mem::transmute(USER_HOST_CALL.write_addr.unwrap()) };
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
