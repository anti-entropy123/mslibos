use core::ptr;

use linked_list_allocator::LockedHeap;
use ms_hostcall::SERVICE_HEAP_SIZE;

use crate::libos::libos;

pub struct LibOSAllocator;

impl LibOSAllocator {
    const fn new() -> Self {
        LibOSAllocator
    }
}

#[global_allocator]
pub static HEAP_ALLOCATOR: LibOSAllocator = LibOSAllocator::new();

unsafe impl core::alloc::GlobalAlloc for LibOSAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        libos!(heap_alloc(layout)).unwrap()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        libos!(heap_dealloc(ptr, layout)).unwrap();
    }
}

// pub static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

// /// Currently, all service will get a static heap region. It is work well but
// /// maybe cause wasting memory.
// pub fn init_heap(heap_start: usize) {
//     unsafe {
//         HEAP_ALLOCATOR
//             .lock()
//             .init(heap_start as *mut u8, SERVICE_HEAP_SIZE)
//     }
// }

#[alloc_error_handler]
// / panic when heap allocation error occurs
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    // let (used_mem, free_mem) = {
    //     let alloctor = HEAP_ALLOCATOR.lock();
    //     (alloctor.used(), alloctor.free())
    // };

    panic!(
        "Heap allocation error, layout = {:?}, used mem={}KB, free mem={}KB",
        layout,
        0, // used_mem >> 10,
        0, // free_mem >> 10
    );
}
