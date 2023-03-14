#![allow(clippy::single_component_path_imports)]
#![allow(unused_imports)]
/// There needs to use ms_std because msvisor will find symbols like
/// set_handler_addr that have been defined in ms_std.
// use ms_std::init_context;
use nix::libc::write;
use std::ffi::c_void;

#[no_mangle]
pub fn host_write(fd: i32, buf: &str) -> isize {
    unsafe { write(fd, buf.as_ptr() as *const c_void, buf.len()) }
}
