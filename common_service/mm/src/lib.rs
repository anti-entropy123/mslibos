#![no_std]

use lazy_static::lazy_static;
use linked_list_allocator::LockedHeap;
use ms_hostcall::mm::MMResult;
use ms_hostcall::SERVICE_HEAP_SIZE;
pub mod faas_buffer;
pub mod mmap;

extern crate alloc;

#[global_allocator]
pub static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

lazy_static! {
    pub static ref MUST_EXEC: bool = {
        let heap_start = ms_std::init_context::ISOLATION_CTX.lock().heap_range.0;
        unsafe {
            HEAP_ALLOCATOR
                .lock()
                .init(heap_start as *mut u8, SERVICE_HEAP_SIZE)
        };
        true
    };
}

#[no_mangle]
fn heap_alloc(layout: core::alloc::Layout) -> MMResult<*mut u8> {
    let _ = *MUST_EXEC;

    unsafe { Ok(alloc::alloc::alloc(layout)) }
}

#[no_mangle]
fn heap_dealloc(ptr: *mut u8, layout: core::alloc::Layout) -> MMResult {
    let _ = *MUST_EXEC;

    unsafe { alloc::alloc::dealloc(ptr, layout) };
    Ok(())
}
