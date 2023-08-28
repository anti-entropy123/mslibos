use core::{
    fmt::Display,
    net::{Ipv4Addr, SocketAddrV4},
};

use alloc::vec::Vec;
use ms_hostcall::types::Fd;

use crate::{io::Write, libos::libos};

#[derive(PartialEq, Debug)]
enum State {
    Connect,
    Request,
    Response,
}

pub struct TcpStream {
    raw_fd: Fd,
    state: State,
}

impl TcpStream {
    pub fn connect(addr: SocketAddr) -> Result<Self, ()> {
        // println!("connect to {}", addr);

        let sockaddrv4 = match addr.inner {
            core::net::SocketAddr::V4(addr) => addr,
            core::net::SocketAddr::V6(_) => todo!(),
        };
        let raw_fd = libos!(connect(sockaddrv4)).expect("libos::connect failed");
        let mut stream = Self {
            raw_fd,
            state: State::Connect,
        };
        stream.state = State::Request;

        Ok(stream)
    }

    pub fn write_all(&mut self, data: &[u8]) -> Result<(), ()> {
        if self.state != State::Request {
            return Err(());
        }

        libos!(write(self.raw_fd, data)).expect("send data failed");
        self.state = State::Response;
        Ok(())
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, ()> {
        if self.state != State::Response {
            return Err(());
        }

        let len = libos!(read(self.raw_fd, buf)).expect("recv data failed");

        Ok(len)
    }
}

impl Write for TcpStream {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_all(s.as_bytes()).expect("write_all failed.");

        Ok(())
    }
}

pub struct SocketAddr {
    inner: core::net::SocketAddr,
}

impl Default for SocketAddr {
    fn default() -> Self {
        Self {
            inner: core::net::SocketAddr::from(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0)),
        }
    }
}

impl Display for SocketAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.inner.fmt(f)
    }
}

impl From<&str> for SocketAddr {
    fn from(value: &str) -> Self {
        let mut sockaddr = SocketAddr::default();

        let (ip, port) = {
            let target_tuple: Vec<&str> = value.split(':').collect();
            let host_str = target_tuple[0];
            let addr = if let Ok(addr) = host_str.parse() {
                addr
            } else {
                libos!(addrinfo(host_str)).expect("wrong tcp domain")
            };

            let port = match target_tuple.get(1) {
                Some(port) => port.parse::<u16>().expect("wrong url"),
                None => 80,
            };

            (addr, port)
        };

        sockaddr.inner = core::net::SocketAddr::from(SocketAddrV4::new(ip, port));

        sockaddr
    }
}
