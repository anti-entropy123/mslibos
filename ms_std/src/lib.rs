#![no_std]
#![feature(linkage)]
#![feature(alloc_error_handler)]
#![feature(ip_in_core)]
#![feature(decl_macro)]
#![feature(concat_idents)]
#![feature(generic_const_exprs)]
#![allow(clippy::result_unit_err)]
#![allow(incomplete_features)]

use agent::{FaaSFuncResult, Zero};

pub mod console;

pub mod agent;
pub mod init_context;
pub mod libos;
pub mod net;
pub mod sync;
pub mod time;

extern crate alloc;

cfg_if::cfg_if! {
    if #[cfg(feature = "alloc_def")] {
        mod heap_alloc;
    }
}

cfg_if::cfg_if! {
    // if #[cfg(all(feature = "unwinding", feature = "panic_def"))] {
    //     compile_error!("must only choose one in 'unwinding' and 'panic_def'");
    // } else
    if #[cfg(feature = "panic_def")] {
        mod panic_def {
            use crate::init_context::isolation_ctx;
            use core::panic::PanicInfo;

            #[panic_handler]
            fn panic_handler(_info: &PanicInfo) -> ! {
                let panic_addr = isolation_ctx().panic_handler;

                let host_panic_handler: unsafe extern "C" fn() -> ! =
                    unsafe { core::mem::transmute(panic_addr) };
                unsafe { host_panic_handler() }
            }

        }
    } else
    if #[cfg(feature = "unwinding")] {
        use unwinding;
    }
}

#[linkage = "weak"]
#[no_mangle]
pub fn main() -> FaaSFuncResult<Zero> {
    panic!("need real main");
}

#[no_mangle]
pub extern "C-unwind" fn rust_main() -> Result<(), ()> {
    panic!("need real main");
    let r = main();
    assert!(r.is_ok());

    Ok(())
}
