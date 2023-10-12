#![no_std]
extern crate alloc;

use nix::{libc::CLOCK_REALTIME, time::clock_gettime};

#[allow(clippy::result_unit_err)]
#[no_mangle]
pub fn get_time() -> Result<u128, ()> {
    let r = clock_gettime(CLOCK_REALTIME.into()).expect("failed");
    let a = r.tv_sec() as u128 * 1_000_000_000;
    let b = r.tv_nsec() as u128;
    Ok(a + b)
}

#[test]
fn get_time_test() {
    let t = get_time().unwrap();
    assert!(t > 1_697_111_969 * 1_000_000_000, "error time: {}", t);
    assert!(t < 1_735_660_800 * 1_000_000_000, "error time: {}", t);
}
