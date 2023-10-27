#![no_std]

use core::alloc::Layout;

extern crate alloc;
use alloc::{collections::BTreeMap, string::String};
use lazy_static::lazy_static;

#[allow(unused)]
#[allow(clippy::single_component_path_imports)]
use ms_std;
use ms_std::println;
use spin::Mutex;

lazy_static! {
    static ref RAW_P: Mutex<BTreeMap<String, (usize, u64)>> = Mutex::new(Default::default());
    static ref DEFAULT_RAW_P: Mutex<Option<(usize, u64)>> = Mutex::new(None);
}

#[allow(clippy::result_unit_err)]
#[no_mangle]
pub fn buffer_alloc(slot: String, l: Layout, fingerprint: u64) -> Result<usize, ()> {
    // println!(
    //     "buffer_alloc: expect size={}, align={}",
    //     l.size(),
    //     l.align()
    // );
    let addr = unsafe { alloc::alloc::alloc(l) };

    if slot.is_empty() {
        *DEFAULT_RAW_P.lock() = Some((addr as usize, fingerprint));
    } else {
        RAW_P.lock().insert(slot, (addr as usize, fingerprint));
    };

    let (used_mem, free_mem) = {
        let alloctor = ms_std::heap_alloc::HEAP_ALLOCATOR.lock();
        (alloctor.used(), alloctor.free())
    };

    // println!(
    //     "alloc addr=0x{:x}, used mem={}KB, free mem={}KB",
    //     addr as usize,
    //     used_mem >> 10,
    //     free_mem >> 10
    // );

    Ok(addr as usize)
}

#[no_mangle]
pub fn access_buffer(slot: String) -> Option<(usize, u64)> {
    // *RAW_P.lock().take()
    if slot.is_empty() {
        DEFAULT_RAW_P.lock().take()
    } else {
        RAW_P.lock().get(&slot).take().copied()
    }
}

#[no_mangle]
pub fn buffer_dealloc(addr: usize, l: Layout) {
    unsafe { alloc::alloc::dealloc(addr as *mut u8, l) }
}
