use std::ffi::c_void;

use libloading::Symbol;
use ms_hostcall::{GetHandlerFunc, HostCall, HostCallID, RustMainFunc, SetHandlerFunc};
use nix::libc::write;

pub type SetHandlerFuncSybmol<'a> = Symbol<'a, SetHandlerFunc>;
pub type GetHandlerFuncSybmol<'a> = Symbol<'a, GetHandlerFunc>;
pub type RustMainFuncSybmol<'a> = Symbol<'a, RustMainFunc>;

pub fn find_host_call(id: HostCallID) -> usize {
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
