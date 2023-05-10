#![no_std]

extern crate alloc;

use alloc::string::String;

use ms_std::{net::TcpStream, println};

#[no_mangle]
pub fn rust_main() -> Result<(), ()> {
    let mut stream = TcpStream::connect("netease.com".into())?;
    stream.write_all(b"GET / HTTP/1.0\r\n\r\n")?;
    let mut buffer = [0; 4096];
    loop {
        let n = stream.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        let response = String::from_utf8_lossy(&buffer[..n]);
        println!("{}", response);
    }
    Ok(())
}
