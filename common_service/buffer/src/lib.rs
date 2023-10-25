#![no_std]

use core::alloc::Layout;

extern crate alloc;
use lazy_static::lazy_static;

#[allow(unused)]
#[allow(clippy::single_component_path_imports)]
use ms_std;
use ms_std::sync::UPSafeCell;

lazy_static! {
    static ref RAW_P: UPSafeCell<Option<(usize, u64)>> = unsafe { UPSafeCell::new(None) };
}

#[allow(clippy::result_unit_err)]
#[no_mangle]
pub fn buffer_alloc(l: Layout, fingerprint: u64) -> Result<usize, ()> {
    // println!("buffer_alloc: l={:?}", l);
    let addr = unsafe { alloc::alloc::alloc(l) };
    *RAW_P.exclusive_access() = Some((addr as usize, fingerprint));

    Ok(addr as usize)
}

#[no_mangle]
pub fn access_buffer() -> Option<(usize, u64)> {
    // *RAW_P.exclusive_access().take()
    *RAW_P.exclusive_access()
}
