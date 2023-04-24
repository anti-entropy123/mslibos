#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use lazy_static::lazy_static;
use ms_std;

struct File;

lazy_static! {
    static ref fd_table: Vec<File> = Vec::new();
}

#[no_mangle]
pub fn host_write(fd: i32, buf: &str) -> isize {
    match fd {
        1 => {
            ms_std::wrapper::UserHostCall::stdout(buf);
            buf.len() as isize
        }
        _ => panic!(),
    }
}
