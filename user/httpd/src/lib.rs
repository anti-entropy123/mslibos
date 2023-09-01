#![no_std]
#![allow(clippy::result_unit_err)]

extern crate alloc;

use ms_std::{
    agent::{FaaSFuncResult as Result, Zero},
    net::TcpListener,
    println,
};

#[no_mangle]
pub fn main() -> Result<Zero> {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        println!("Connection established!");
    }

    Ok(Zero::default().into())
}
