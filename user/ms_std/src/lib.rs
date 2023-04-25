#![no_std]
#![feature(lang_items)]
// #![feature(default_alloc_error_handler)]
// #![feature(panic_info_message)]
#![feature(alloc_error_handler)]
extern crate alloc;

pub mod console;
mod heap_alloc;
pub mod init_context;
mod sync;
mod syscall_wrapper;

use core::panic::PanicInfo;

pub use syscall_wrapper::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
