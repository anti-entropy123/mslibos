#![no_std]
#![feature(lang_items)]
// #![feature(default_alloc_error_handler)] // have been stable.
#![feature(linkage)]
#![feature(alloc_error_handler)]
extern crate alloc;

pub mod console;
mod heap_alloc;
pub mod init_context;
mod sync;
pub mod wrapper;

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
