use std::{io::Write, net::TcpStream};

const DB_AGENT: &str = "192.168.69.100:3000";

pub fn memcached_get(payload: String) -> String {
    let mut stream =
        TcpStream::connect("192.168.69.100:9999".into()).expect("connect to db agent failed.");

    stream
        .write_all(b"GET / HTTP/1.0\r\n\r\n")
        .expect("send request failed.");

    let mut buffer = [0; 4096];
    // println!("send data finish.");
    loop {
        let n = stream.read(&mut buffer).expect("read failed");
        if n == 0 {
            break;
        }
        let response = String::from_utf8_lossy(&buffer[..n]);
        // println!("{}", response);
    }

    String::new()
}
