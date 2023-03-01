pub trait HostCall {
    fn host_write(fd: i32, buf: &str) -> isize;
}

#[repr(C)]
pub enum HostCallID {
    Write,
}

pub type FindHostCall = fn(HostCallID) -> usize;
