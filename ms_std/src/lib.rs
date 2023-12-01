#![no_std]
#![feature(lang_items)]
#![feature(linkage)]
#![feature(alloc_error_handler)]
#![feature(ip_in_core)]
#![feature(decl_macro)]
#![feature(concat_idents)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![feature(const_maybe_uninit_zeroed)]

use agent::FaaSFuncResult;
use alloc::{collections::BTreeMap, string::String};

pub mod console;

pub mod agent;
pub mod fs;
pub mod init_context;
pub mod io;
pub mod libos;
pub mod mm;
pub mod net;
pub mod sym_patch;
pub mod sync;
pub mod time;

extern crate alloc;

#[cfg(feature = "alloc_def")]
pub mod heap_alloc;

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
        }
    }
}

#[linkage = "weak"]
#[no_mangle]
pub fn main(_: BTreeMap<String, String>) -> FaaSFuncResult<()> {
    panic!("need real main");
}

#[no_mangle]
pub fn rust_main(args: BTreeMap<String, String>) -> Result<(), String> {
    #[cfg(feature = "unwinding")]
    {
        let result = unwinding::panic::catch_unwind(|| main(args));

        match result {
            Ok(func_res) => {
                if let Err(e) = func_res {
                    Err(alloc::format!("function exec error: {}", e.msg()))?;
                }
            }
            Err(e) => {
                core::mem::forget(e);
                Err(alloc::format!("catch user function panic."))?;
            }
        }
    }
    #[cfg(not(feature = "unwinding"))]
    {
        let result = main(args);

        if let Err(e) = result {
            Err(alloc::format!("function exec error: {}", e.msg()))?
        };
    }
    Ok(())
}
