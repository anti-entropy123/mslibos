#![no_std]
// #[deny(unsafe_code)]
// #[allow(no_mangle_generic_items)]

use ms_std::println;

#[no_mangle]
pub extern "C" fn rust_main() {
    println!("Hello, world!");
    // UserHostCall::host_write(1, "hello_world!\n");
}
