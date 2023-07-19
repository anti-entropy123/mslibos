#![no_std]

use core::alloc::Layout;

extern crate alloc;

#[allow(unused)]
#[allow(clippy::single_component_path_imports)]
use ms_std;

#[allow(clippy::result_unit_err)]
#[no_mangle]
pub fn buffer_alloc(l: Layout) -> Result<usize, ()> {
    let addr = unsafe { alloc::alloc::alloc(l) };

    Ok(addr as usize)
}
