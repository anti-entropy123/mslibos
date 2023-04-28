//! This crate support the defination of symbols `set_handler_addr`
//! and `get_handler_addr`. These two symbols will be lookup when 
//! initialize service. 
//! To some service that depent `std`, they would use this crate directly 
//! rather than `ms_std`.

use core::cell::{Ref, RefMut};

use ms_hostcall::{types::HostCallResult as HCResult, IsolationContext};

use lazy_static::lazy_static;

use crate::{heap_alloc::init_heap, sync::UPSafeCell};

lazy_static! {
    // In fact, isolation_ctx should be readonly, so don't have to
    // use Mutex or UPSafeCell.
    pub static ref ISOLATION_CTX: UPSafeCell<IsolationContext> = UPSafeCell::default();
}

/// This is a non-pub function because it should not be init in other file.
fn isolation_ctx_mut() -> RefMut<'static, IsolationContext> {
    ISOLATION_CTX.exclusive_access()
}

pub fn isolation_ctx() -> Ref<'static, IsolationContext> {
    let ctx = ISOLATION_CTX.access();
    if ctx.panic_handler == 0 {
        panic!("uninit")
    }
    ctx
}

#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub extern "C" fn set_handler_addr(ctx: IsolationContext) -> HCResult {
    let mut isol_ctx = isolation_ctx_mut();
    if isol_ctx.find_handler != 0 {
        panic!();
        // return Err(HCError::HasBeenSet);
    };
    *isol_ctx = ctx;

    init_heap(ctx.heap_range.0);
    Ok(())
}

#[no_mangle]
pub extern "C" fn get_handler_addr() -> usize {
    let isol_ctx = isolation_ctx();

    isol_ctx.find_handler
}
