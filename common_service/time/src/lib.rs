#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use nix::{
    libc::{timespec, CLOCK_REALTIME},
    time::clock_gettime,
};

#[no_mangle]
pub fn get_time() -> Result<u128, String> {
    let r = clock_gettime(CLOCK_REALTIME.into()).map_err(|e| e.to_string())?;
    let a = r.tv_sec() as u128 * 1_000_000_000;
    let b = r.tv_nsec() as u128;
    Ok(a + b)
}

#[no_mangle]
pub fn host_nanosleep(sec: u64, nsec: u64) {
    let mut ts = timespec {
        tv_sec: sec as i64,
        tv_nsec: nsec as i64,
    };
    let ts_ptr = &mut ts as *mut _;

    unsafe { nix::libc::nanosleep(ts_ptr, ts_ptr) };
}

#[test]
fn get_time_test() {
    let t = get_time().unwrap();
    assert!(t > 1_697_111_969 * 1_000_000_000, "error time: {}", t);
    assert!(t < 1_735_660_800 * 1_000_000_000, "error time: {}", t);
}

#[test]
fn test_nanosleep() {
    host_nanosleep(1, 0)
}
