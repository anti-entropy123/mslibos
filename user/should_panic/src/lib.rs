#![no_std]
use ms_std;

#[no_mangle]
pub extern "C" fn rust_main() {
    panic!("I must panic!")
}
