use core::{
    fmt::Display,
    net::{Ipv4Addr, SocketAddrV4},
};

#[cfg(feature = "no_std")]
use alloc::vec::Vec;
use ms_hostcall::types::{NetDevice, NetIface};

use crate::{
    libos::{self},
    println,
};

#[derive(PartialEq, Debug)]
enum State {
    Connect,
    Request,
    Response,
}

pub struct TcpStream {
    device: NetDevice,
    iface: NetIface,

    state: State,
}

impl TcpStream {
    pub fn connect(addr: SocketAddr) -> Result<Self, ()> {
        // println!("connect to {}", addr);

        let mut stream = Self {
            device: addr.device,
            iface: addr.iface,
            state: State::Connect,
        };
        let sockaddrv4 = match addr.inner {
            core::net::SocketAddr::V4(addr) => addr,
            core::net::SocketAddr::V6(_) => todo!(),
        };
        libos::connect(&mut stream.iface, sockaddrv4).expect("libos::connect failed");
        stream.state = State::Request;

        Ok(stream)
    }

    pub fn write_all(&mut self, data: &[u8]) -> Result<(), ()> {
        assert_eq!(self.state, State::Request);
        libos::send(&mut self.device, &mut self.iface, data).expect("send data failed");
        self.state = State::Response;
        Ok(())
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, ()> {
        assert_eq!(self.state, State::Response);
        let len = libos::recv(&mut self.device, &mut self.iface, buf).expect("recv data failed");

        Ok(len)
    }
}

pub struct SocketAddr {
    device: NetDevice,
    iface: NetIface,

    inner: core::net::SocketAddr,
}

impl Default for SocketAddr {
    fn default() -> Self {
        let (device, iface) = libos::init_dev().expect("init net dev failed.");

        Self {
            device,
            iface,
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

        let tcp_addr: Vec<&str> = value.split(':').collect();
        let (ip, port) = {
            let addr: Ipv4Addr = if let Ok(addr) = tcp_addr[0].parse() {
                addr
            } else {
                libos::addr_info(&mut sockaddr.device, &mut sockaddr.iface, tcp_addr[0])
                    .expect("wrong tcp domain")
            };

            let port = if tcp_addr.get(1).is_none() {
                80
            } else {
                tcp_addr[1].parse::<u32>().expect("wrong tcp port")
            };

            (addr, port as u16)
        };

        sockaddr.inner = core::net::SocketAddr::from(SocketAddrV4::new(ip, port));

        sockaddr
    }
}
