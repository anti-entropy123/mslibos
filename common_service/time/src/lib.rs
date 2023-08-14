#![no_std]
extern crate alloc;

use nix::{libc::CLOCK_MONOTONIC, time::clock_gettime};

#[allow(clippy::result_unit_err)]
#[no_mangle]
pub fn get_time() -> Result<u128, ()> {
    let r = clock_gettime(CLOCK_MONOTONIC.into()).expect("failed");
    Ok(r.tv_sec() as u128 * 1_000_000_000 + r.tv_nsec() as u128)
}
