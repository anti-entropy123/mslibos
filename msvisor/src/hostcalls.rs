use std::ffi::c_void;

use ms_hostcall::{HostCall, HostCallID};
use nix::libc::write;

#[no_mangle]
pub extern "C" fn find_host_call(id: HostCallID) -> usize {
    let addr = match id {
        HostCallID::Write => RustHostCall::host_write as usize,
    };
    log::debug!("host_write addr = 0x{:x}", addr);
    addr
}

struct RustHostCall;
impl HostCall for RustHostCall {
    fn host_write(fd: i32, buf: &str) -> isize {
        unsafe { write(fd, buf.as_ptr() as *const c_void, buf.len()) }
    }
}
