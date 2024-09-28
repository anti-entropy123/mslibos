extern crate alloc;

use core::alloc::Layout;

use alloc::{borrow::ToOwned, string::String};

use ms_hostcall::mm::MMResult;

use hashbrown::HashMap;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    static ref BUFFER_REGISTER: Mutex<HashMap<String, (usize, u64)>> = Mutex::new(HashMap::new());
}



#[no_mangle]
pub fn buffer_alloc(slot: &str, l: Layout, fingerprint: u64) -> MMResult<usize> {
    let addr = unsafe { alloc::alloc::alloc(l) };

    BUFFER_REGISTER
        .lock()
        .insert(slot.to_owned(), (addr as usize, fingerprint));

    Ok(addr as usize)
}

#[no_mangle]
pub fn access_buffer(slot: &str) -> Option<(usize, u64)> {
    // *RAW_P.lock().take()

    BUFFER_REGISTER.lock().get(slot).take().copied()
}

#[no_mangle]
pub fn buffer_dealloc(addr: usize, l: Layout) {
    unsafe { alloc::alloc::dealloc(addr as *mut u8, l) }
}
