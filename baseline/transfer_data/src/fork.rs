use nix::libc::{self, SIGCHLD};

pub type CloneCb = Box<dyn FnMut() -> i32>;

pub fn clone3(mut cb: CloneCb, flags: u64) -> Result<i32, String> {
    #[repr(C)]
    struct Clone3Args {
        flags: u64,
        pidfd: u64,
        child_tid: u64,
        parent_tid: u64,
        exit_signal: u64,
        stack: u64,
        stack_size: u64,
        tls: u64,
        set_tid: u64,
        set_tid_size: u64,
        cgroup: u64,
    }
    let mut args = Clone3Args {
        flags,
        pidfd: 0,
        child_tid: 0,
        parent_tid: 0,
        exit_signal: SIGCHLD as u64,
        stack: 0,
        stack_size: 0,
        tls: 0,
        set_tid: 0,
        set_tid_size: 0,
        cgroup: 0,
    };
    let args_ptr = &mut args as *mut Clone3Args;
    let args_size = std::mem::size_of::<Clone3Args>();
    match unsafe { libc::syscall(libc::SYS_clone3, args_ptr, args_size) } {
        -1 => Err(nix::Error::last().to_string()),
        0 => {
            std::process::exit(cb());
        }
        ret if ret >= 0 => Ok(ret as i32),
        _ret => Err("unknown errno".to_owned()),
    }
}
