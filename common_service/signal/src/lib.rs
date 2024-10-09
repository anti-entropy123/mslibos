pub use ms_std;
use nc::{self, sigset_t};
use ms_hostcall::signal::SigAction;

#[no_mangle]
pub fn libos_sigaction(signum: i32, act: *const SigAction, old_act: *const SigAction) -> i32 {
    let ret = unsafe {
        let act = nc::sigaction_t {
            sa_handler: (*act).sa_handler as nc::sighandler_t,
            sa_flags: (*act).sa_flags as usize,
            sa_mask: sigset_t { sig: (*act).sa_mask.sig },
            ..nc::sigaction_t::default()
        };
        if old_act.is_null() {
            nc::rt_sigaction(signum, Some(&act), None)
        } else {
            let mut old_act = nc::sigaction_t {
                sa_handler: (*old_act).sa_handler as nc::sighandler_t,
                sa_flags: (*old_act).sa_flags as usize,
                sa_mask: sigset_t { sig: (*old_act).sa_mask.sig },
                ..nc::sigaction_t::default()
            };
            nc::rt_sigaction(signum, Some(&act), Some(&mut old_act))
        }
    };
    let ret = match ret {
        Ok(_) => 0,
        Err(errno) => -errno,
    };
    ret
}