extern crate alloc;

use core::{alloc::Layout, ptr::NonNull};

use alloc::{borrow::ToOwned, string::String};

use linked_list_allocator::LockedHeap;
use ms_hostcall::{mm::MMResult, SERVICE_HEAP_SIZE};

use hashbrown::HashMap;
use lazy_static::lazy_static;
use ms_std::println;
use spin::Mutex;

lazy_static! {
    static ref BUFFER_REGISTER: Mutex<HashMap<String, (usize, u64)>> = Mutex::new(HashMap::new());
    static ref BUFFER_ALLOCATOR: LockedHeap = unsafe {
        LockedHeap::new(
            ms_std::init_context::ISOLATION_CTX.lock().heap_range.1 as *mut u8,
            SERVICE_HEAP_SIZE / 2,
        )
    };
}

#[no_mangle]
pub fn buffer_alloc(slot: &str, l: Layout, fingerprint: u64) -> MMResult<usize> {
    let addr = BUFFER_ALLOCATOR.lock().allocate_first_fit(l).unwrap();
    let addr = addr.as_ptr() as usize;

    BUFFER_REGISTER
        .lock()
        .insert(slot.to_owned(), (addr, fingerprint));

    println!("buffer_alloc addr=0x{:x}", addr);

    Ok(addr)
}

#[no_mangle]
pub fn access_buffer(slot: &str) -> Option<(usize, u64)> {
    // *RAW_P.lock().take()
    BUFFER_REGISTER.lock().get(slot).take().copied()
}

#[no_mangle]
pub fn buffer_dealloc(addr: usize, l: Layout) {
    unsafe {
        BUFFER_ALLOCATOR
            .lock()
            .deallocate(NonNull::new(addr as *mut u8).unwrap(), l)
    }
}
