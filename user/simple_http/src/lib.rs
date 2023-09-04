#![no_std]
#![allow(clippy::result_unit_err)]

extern crate alloc;

use alloc::string::String;

use ms_std::{
    agent::{FaaSFuncResult as Result, Zero},
    net::TcpStream,
    println,
};

#[no_mangle]
pub fn main() -> Result<Zero> {
    let mut stream = TcpStream::connect("baidu.com".into())?;
    stream.write_all(b"GET / HTTP/1.0\r\n\r\n")?;
    let mut buffer = [0; 4096];
    loop {
        let n = stream.read(&mut buffer).expect("read failed");
        if n == 0 {
            break;
        }
        let response = String::from_utf8_lossy(&buffer[..n]);
        println!("{}", response);
    }

    Ok(Zero::default().into())
}
