use core::{
    fmt::Display,
    net::{Ipv4Addr, SocketAddrV4},
};

use alloc::vec::Vec;

use crate::{libos, println};

pub struct TcpStream {}

impl TcpStream {
    pub fn connect(addr: SocketAddr) -> Result<Self, ()> {
        println!("connect to {}", addr);
        Err(())
    }

    pub fn write_all(&mut self, _data: &[u8]) -> Result<(), ()> {
        Err(())
    }

    pub fn read(&self, _buf: &[u8]) -> Result<usize, ()> {
        Err(())
    }
}

pub struct SocketAddr {
    inner: core::net::SocketAddr,
}

impl Display for SocketAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.inner.fmt(f)
    }
}

impl From<&str> for SocketAddr {
    fn from(value: &str) -> Self {
        let tcp_addr: Vec<&str> = value.split(":").collect();
        let (ip, port) = {
            let addr: Ipv4Addr = if let Ok(addr) = tcp_addr[0].parse() {
                addr
            } else {
                libos::addr_info(tcp_addr[0]).expect("wrong tcp domain")
            };

            let port = if tcp_addr.get(1).is_none() {
                80
            } else {
                tcp_addr[1].parse::<u32>().expect("wrong tcp port")
            };

            (addr, port as u16)
        };
        let sock_addr_v4 = SocketAddrV4::new(ip, port);
        Self {
            inner: core::net::SocketAddr::from(sock_addr_v4),
        }
    }
}
