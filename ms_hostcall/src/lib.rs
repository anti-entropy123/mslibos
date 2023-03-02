pub trait HostCall {
    fn host_write(fd: i32, buf: &str) -> isize;
}

#[repr(C)]
pub enum HostCallID {
    Write,
}

pub type FindHostCall = fn(HostCallID) -> usize;
pub type SetHandlerFunc = unsafe extern "C" fn(usize) -> Result<(), ()>;
pub type GetHandlerFunc = unsafe extern "C" fn() -> usize;
pub type RustMainFunc = unsafe extern "C" fn() -> ();
