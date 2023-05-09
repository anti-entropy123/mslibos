use core::{
    fmt::Display,
    net::{Ipv4Addr, SocketAddrV4},
    str::FromStr,
};

#[cfg(feature = "no_std")]
use alloc::vec::Vec;
use ms_hostcall::types::{NetDevice, NetIface};
use url::Url;

use crate::libos;

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
        let value = if value.starts_with("http") {
            value.to_owned()
        } else {
            format!("http://{}", value)
        };
        let url = Url::from_str(&value).expect("wrong url");

        let mut sockaddr = SocketAddr::default();

        let (ip, port) = {
            let host_str = url.host_str().expect("wrong http url");
            let addr = if let Ok(addr) = host_str.parse() {
                addr
            } else {
                libos::addr_info(&mut sockaddr.device, &mut sockaddr.iface, host_str)
                    .expect("wrong tcp domain")
            };

            (addr, url.port().unwrap_or(80))
        };

        sockaddr.inner = core::net::SocketAddr::from(SocketAddrV4::new(ip, port));

        sockaddr
    }
}
