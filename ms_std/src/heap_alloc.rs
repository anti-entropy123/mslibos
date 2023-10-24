use linked_list_allocator::LockedHeap;
use ms_hostcall::SERVICE_HEAP_SIZE;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Currently, all service will get a static heap region. It is work well but
/// maybe cause wasting memory.
pub fn init_heap(heap_start: usize) {
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(heap_start as *mut u8, SERVICE_HEAP_SIZE)
    }
}

#[alloc_error_handler]
/// panic when heap allocation error occurs
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}
