use core::net::SocketAddrV4;

use thiserror_no_std::Error;

use crate::{
    err::LibOSResult,
    types::{Size, SockFd},
};

pub type SmoltcpAddrInfoFunc = fn(&str) -> Result<core::net::Ipv4Addr, ()>;
pub type SmoltcpConnectFunc = fn(SocketAddrV4) -> Result<SockFd, ()>;
pub type SmoltcpSendFunc = fn(SockFd, &[u8]) -> Result<(), ()>;
pub type SmoltcpRecvFunc = fn(SockFd, &mut [u8]) -> Result<Size, ()>;
pub type SmoltcpBindFunc = fn(SocketAddrV4) -> LibOSResult<SockFd>;
pub type SmoltcpAcceptFunc = fn(SockFd) -> LibOSResult<SockFd>;
pub type SmoltcpCloseFunc = fn(SockFd) -> LibOSResult<()>;

pub type SmoltcpResult<T> = Result<T, SmoltcpError>;

#[derive(Error)]
pub enum SmoltcpError {}
