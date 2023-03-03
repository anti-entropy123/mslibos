#![allow(clippy::single_component_path_imports)]
#![allow(unused_imports)]
use ms_std;
use nix::libc::write;
use std::ffi::c_void;

#[no_mangle]
pub fn host_write(fd: i32, buf: &str) -> isize {
    unsafe { write(fd, buf.as_ptr() as *const c_void, buf.len()) }
}
