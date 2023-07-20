#![no_std]

use core::alloc::Layout;

extern crate alloc;
use lazy_static::lazy_static;

#[allow(unused)]
#[allow(clippy::single_component_path_imports)]
use ms_std;
use ms_std::sync::UPSafeCell;

lazy_static! {
    static ref RAW_P: UPSafeCell<Option<usize>> = unsafe { UPSafeCell::new(None) };
}

#[allow(clippy::result_unit_err)]
#[no_mangle]
pub fn buffer_alloc(l: Layout) -> Result<usize, ()> {
    let addr = unsafe { alloc::alloc::alloc(l) };
    *RAW_P.exclusive_access() = Some(addr as usize);

    Ok(addr as usize)
}

#[no_mangle]
pub fn access_buffer() -> Option<usize> {
    let r = *RAW_P.access();
    *RAW_P.exclusive_access() = None;
    r
}
