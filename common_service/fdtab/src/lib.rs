#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use lazy_static::lazy_static;
use ms_std::{self, libos::libos};

struct File {}

lazy_static! {
    static ref _FD_TABLE: Vec<File> = Vec::new();
}

#[no_mangle]
pub fn host_write(fd: i32, buf: &str) -> isize {
    match fd {
        1 | 2 => {
            libos!(stdout(buf));
            buf.len() as isize
        }
        _ => panic!(),
    }
}
