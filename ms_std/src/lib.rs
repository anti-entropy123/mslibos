#![no_std]
#![feature(lang_items)]
#![feature(linkage)]
#![feature(alloc_error_handler)]
#![feature(ip_in_core)]

extern crate alloc;

pub mod console;
pub mod init_context;
pub mod libos;
pub mod net;

mod heap_alloc;
mod sync;

use core::panic::PanicInfo;

use init_context::isolation_ctx;

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    let panic_addr = isolation_ctx().panic_handler;

    let host_panic_handler: unsafe extern "C" fn() -> ! =
        unsafe { core::mem::transmute(panic_addr) };
    unsafe { host_panic_handler() }
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[linkage = "weak"]
#[no_mangle]
pub fn rust_main() {
    panic!("need real rust_main");
}
