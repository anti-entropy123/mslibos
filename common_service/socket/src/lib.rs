#![feature(ip_in_core)]

use core::net::Ipv4Addr;
// pub use ms_std::init_context;

#[no_mangle]
pub fn addrinfo(_name: &str) -> Result<Ipv4Addr, ()> {
    Ok(Ipv4Addr::new(5, 6, 7, 8))
}

#[no_mangle]
pub fn connect() {}

#[no_mangle]
pub fn send() {}

#[no_mangle]
pub fn recv() {}

#[no_mangle]
pub fn close_tcp() {}
