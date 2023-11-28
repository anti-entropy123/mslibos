use core::alloc::Layout;

use alloc::borrow::ToOwned;
use ms_hostcall::mm::MMResult;

use super::{BUFFER_REGISTER, DEFAULT_BUFFER_ENTRY};

extern crate alloc;

#[no_mangle]
pub fn buffer_alloc(slot: &str, l: Layout, fingerprint: u64) -> MMResult<usize> {
    // println!(
    //     "buffer_alloc: expect size={}, align={}",
    //     l.size(),
    //     l.align()
    // );
    let addr = unsafe { alloc::alloc::alloc(l) };

    if slot.is_empty() {
        *DEFAULT_BUFFER_ENTRY.lock() = Some((addr as usize, fingerprint));
    } else {
        BUFFER_REGISTER
            .lock()
            .insert(slot.to_owned(), (addr as usize, fingerprint));
    };

    // let (used_mem, free_mem) = {
    //     let alloctor = ms_std::heap_alloc::HEAP_ALLOCATOR.lock();
    //     (alloctor.used(), alloctor.free())
    // };

    // println!(
    //     "alloc addr=0x{:x}, used mem={}KB, free mem={}KB",
    //     addr as usize,
    //     used_mem >> 10,
    //     free_mem >> 10
    // );

    Ok(addr as usize)
}

#[no_mangle]
pub fn access_buffer(slot: &str) -> Option<(usize, u64)> {
    // *RAW_P.lock().take()
    if slot.is_empty() {
        DEFAULT_BUFFER_ENTRY.lock().take()
    } else {
        BUFFER_REGISTER.lock().get(slot).take().copied()
    }
}

#[no_mangle]
pub fn buffer_dealloc(addr: usize, l: Layout) {
    unsafe { alloc::alloc::dealloc(addr as *mut u8, l) }
}
