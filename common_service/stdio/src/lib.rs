use nix::libc::write;
use std::ffi::c_void;

#[no_mangle]
pub fn host_stdout(buf: &str) -> isize {
    unsafe { write(1, buf.as_ptr() as *const c_void, buf.len()) }
}
