#![no_std]

use ms_std::println;

#[no_mangle]
pub extern "C" fn rust_main() {
    println!("Hello, world!");
}
