#![no_std]
#![allow(clippy::result_unit_err)]

extern crate alloc;

use alloc::string::String;

use ms_std::{agent::FaaSFuncResult as Result, net::TcpStream, println};

#[no_mangle]
pub fn main() -> Result<()> {
    let mut stream = TcpStream::connect("www.baidu.com".into())?;
    println!("tcp connection created.");
    stream.write_all(b"GET / HTTP/1.0\r\n\r\n")?;
    let mut buffer = [0; 4096];
    // println!("send data finish.");
    loop {
        let n = stream.read(&mut buffer).expect("read failed");
        if n == 0 {
            break;
        }
        let response = String::from_utf8_lossy(&buffer[..n]);
        println!("{}", response);
    }

    Ok(().into())
}
