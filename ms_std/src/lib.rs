#![no_std]
#![feature(lang_items)]
#![feature(linkage)]
#![feature(alloc_error_handler)]
#![feature(ip_in_core)]
#![feature(decl_macro)]
#![feature(concat_idents)]
#![feature(generic_const_exprs)]
#![allow(clippy::result_unit_err)]
#![allow(incomplete_features)]

use agent::FaaSFuncResult;
use alloc::{collections::BTreeMap, string::String};

pub mod console;

pub mod agent;
pub mod fs;
pub mod init_context;
pub mod io;
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
    if #[cfg(all(feature = "unwinding", feature = "panic_def"))] {
        compile_error!("must only choose one in 'unwinding' and 'panic_def'");
    } else
    if #[cfg(feature = "panic_def")] {
        mod panic_def {
            use core::panic::PanicInfo;

            use crate::init_context::isolation_ctx;

            #[panic_handler]
            fn panic_handler(_info: &PanicInfo) -> ! {
                let panic_addr = isolation_ctx().panic_handler;

                let host_panic_handler: unsafe extern "C" fn() -> ! =
                    unsafe { core::mem::transmute(panic_addr) };
                unsafe { host_panic_handler() }
            }

            #[lang = "eh_personality"]
            extern "C" fn eh_personality() {}

            // If remove this line, will have compile error: "undefined
            // symbol: _Unwind_Resume"
            #[allow(non_snake_case)]
            #[linkage = "weak"]
            #[no_mangle]
            pub fn _Unwind_Resume() {
                use crate::println;
                println!("libos: _unwind_resume")
            }
        }
    }
}

#[linkage = "weak"]
#[no_mangle]
pub fn main(_: BTreeMap<String, String>) -> FaaSFuncResult<()> {
    panic!("need real main");
}

#[no_mangle]
pub fn rust_main(args: BTreeMap<String, String>) -> Result<(), ()> {
    #[cfg(feature = "unwinding")]
    {
        use unwinding::panic;
        let result = panic::catch_unwind(|| main(args));

        if let Err(ref e) = result {
            println!("error: {:#?}", e);
            return Err(());
        }
    }
    #[cfg(not(feature = "unwinding"))]
    {
        let r = main(args);
        assert!(r.is_ok());
    }
    Ok(())
}
