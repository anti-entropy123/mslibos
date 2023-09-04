#![no_std]
#![allow(clippy::result_unit_err)]

extern crate alloc;

use alloc::string::String;
use ms_std::{
    agent::{FaaSFuncResult as Result, Zero},
    net::{TcpListener, TcpStream},
    println,
};

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 4096];

    let n = stream.read(&mut buffer).expect("tcp connect read failed");

    println!("Request: {:#?}", String::from_utf8_lossy(&buffer[..n]));
}

#[no_mangle]
pub fn main() -> Result<Zero> {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        println!("Connection established!");
        handle_connection(stream);
    }

    Ok(Zero::default().into())
}
