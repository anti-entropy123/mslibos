#![no_std]
#![feature(lang_items)]
#![feature(linkage)]
#![feature(alloc_error_handler)]
#![feature(ip_in_core)]
#![feature(decl_macro)]
#![feature(concat_idents)]
#![allow(clippy::result_unit_err)]

#[cfg(feature = "no_std")]
extern crate alloc;

pub mod console;

pub mod init_context;
pub mod libos;
pub mod net;
pub mod sync;

#[cfg(feature = "no_std")]
mod heap_alloc;

use unwinding::panic;

// #[cfg(feature = "no_std")]
// use {core::panic::PanicInfo, init_context::isolation_ctx};

// #[cfg(feature = "no_std")]
// #[panic_handler]
// fn panic_handler(_info: &PanicInfo) -> ! {
//     let panic_addr = isolation_ctx().panic_handler;

//     let host_panic_handler: unsafe extern "C" fn() -> ! =
//         unsafe { core::mem::transmute(panic_addr) };
//     unsafe { host_panic_handler() }
// }

// #[cfg(feature = "no_std")]
// #[lang = "eh_personality"]
// extern "C" fn eh_personality() {}

#[linkage = "weak"]
#[no_mangle]
pub fn main() {
    panic!("need real main");
}

#[no_mangle]
pub fn rust_main() -> Result<(), ()> {
    let result = panic::catch_unwind(main);

    if let Err(ref e) = result {
        println!("error: {:#?}", e)
    }

    result.map_err(|_| ())
}
