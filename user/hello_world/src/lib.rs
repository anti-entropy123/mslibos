#![no_std]
use ms_std::println;
pub use ms_std::set_handler_addr;

#[no_mangle]
pub extern "C" fn rust_main() {
    println!("Hello, world!");
}
