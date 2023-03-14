use ms_hostcall::{types::HostCallResult as HCResult, IsolationContext};

use crate::{syscall_wrapper::USER_HOST_CALL, heap_alloc::init_heap};

#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub extern "C" fn set_handler_addr(ctx: IsolationContext) -> HCResult {
    let mut hostcalls = USER_HOST_CALL.exclusive_access();
    if hostcalls.isolation_ctx.is_some() {
        panic!();
        // return Err(HCError::HasBeenSet);
    };
    hostcalls.isolation_ctx = Some(ctx);
    init_heap(ctx.heap_range.0);
    Ok(())
}

#[no_mangle]
pub extern "C" fn get_handler_addr() -> usize {
    USER_HOST_CALL
        .exclusive_access()
        .isolation_ctx
        .unwrap()
        .find_handler
}
