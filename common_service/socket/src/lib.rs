#![feature(ip_in_core)]
#![allow(clippy::result_unit_err)]

mod drop_tap;
mod setup_tap;

use core::net::Ipv4Addr;
use std::{cmp::min, net::SocketAddrV4, os::fd::AsRawFd, sync::Mutex};

use lazy_static::lazy_static;
use smoltcp::{
    iface::{Config, Interface, SocketHandle, SocketSet},
    phy::{wait as phy_wait, Device, Medium, TunTapInterface},
    socket::{
        dns::{self, GetQueryResultError},
        tcp::{self, Socket, State},
    },
    time::Instant,
    wire::{DnsQueryType, EthernetAddress, IpAddress, IpCidr, Ipv4Address},
};

use crate::setup_tap::exec_tap_setup;
use ms_hostcall::{
    err::{LibOSErr, LibOSResult},
    types::{NetdevName, Size, SockFd},
};
use ms_std::init_context;

thread_local! {
    static DEVICE: Mutex<TunTapInterface> = {
        let mut netdev_name = NETDEV_NAME.lock().unwrap();
        if netdev_name.is_none() {
            *netdev_name = Some(NetdevName{
                name: format!("tap-{}", init_context::isolation_ctx().isol_id),
                subnet: Ipv4Addr::new(192, 168, 69, 0),
                mask: 24
            });
        };

        exec_tap_setup(netdev_name.as_ref().unwrap()).expect("setup tap device failed.");

        Mutex::from(TunTapInterface::new(&netdev_name.as_ref().unwrap().name, Medium::Ethernet).unwrap())
    };

    static TAP_RAW_FD: i32 = DEVICE.with(|device| device.lock().unwrap().as_raw_fd());
}

lazy_static! {
    /// NETDEV_NAME is tap device name, such as tap-42, and also index the ip
    /// address that libos should use.
    ///
    /// Here use lazy_static because device name and ip address are allocated
    /// when runtime.
    ///
    /// Here use Option<> because the `socket` module may never be used so in
    /// this case shouldn't ask for a NetdevName.
    pub static ref NETDEV_NAME: Mutex<Option<NetdevName>> = Mutex::default();

    static ref SOCKETS: Mutex<SocketSet<'static>> = Mutex::new(SocketSet::new(vec![]));

    static ref IFACE: Mutex<Interface> = {
        let mut iface = DEVICE.with(|device_tls| {
            let mut device = device_tls.lock().unwrap();
            let mut config = match device.capabilities().medium {
                Medium::Ethernet => {
                    Config::new(EthernetAddress([0x02, 0x00, 0x00, 0x00, 0x00, 0x01]).into())
                }
                Medium::Ip => Config::new(smoltcp::wire::HardwareAddress::Ip),
                // Medium::Ieee802154 => todo!(),
            };

            config.random_seed = rand::random();
            Interface::new(config, &mut (*device), Instant::now())
        });

        iface.update_ip_addrs(|ip_addrs| {
            ip_addrs
                .push(IpCidr::new(
                    IpAddress::v4(192, 168, 69, init_context::isolation_ctx().isol_id as u8),
                    24,
                ))
                .unwrap();
        });
        // The default ipv4 route is equal to ip address of tap device. (Look scripts/make_tap.sh).
        iface
            .routes_mut()
            .add_default_ipv4_route(Ipv4Address::new(192, 168, 69, 100))
            .unwrap();

        Mutex::new(iface)
    };
}

fn get_tap_raw_fd() -> i32 {
    TAP_RAW_FD.with(|fd| *fd)
}

fn from_sockfd(handle: SockFd) -> SocketHandle {
    unsafe { core::mem::transmute(handle as usize) }
}

fn to_sockfd(handle: SocketHandle) -> SockFd {
    let fd: usize = unsafe { core::mem::transmute(handle) };
    fd as SockFd
}

fn iface_poll(iface: &mut Interface, sockets: &mut SocketSet) -> Instant {
    let timestamp = Instant::now();

    DEVICE.with(|device_tls| {
        let mut device = device_tls.lock().unwrap();
        iface.poll(timestamp, &mut (*device), sockets)
    });

    timestamp
}

fn try_phy_wait(
    timestamp: Instant,
    iface: &mut Interface,
    sockets: &mut SocketSet,
) -> LibOSResult<()> {
    phy_wait(get_tap_raw_fd(), iface.poll_delay(timestamp, sockets))
        .map_err(|_| LibOSErr::PhyWaitErr)
}

#[no_mangle]
pub fn addrinfo(name: &str) -> Result<Ipv4Addr, ()> {
    // println!("Device raw_fd is {}", fd);

    let mut iface = IFACE.lock().unwrap();

    let (dns_handle, query) = {
        let servers = &[Ipv4Address::new(8, 8, 8, 8).into()];
        let dns_socket = dns::Socket::new(servers, vec![]);

        let mut sockets = SOCKETS.lock().unwrap();
        let dns_handle: SocketHandle = sockets.add(dns_socket);
        let dns_socket = sockets.get_mut::<dns::Socket>(dns_handle);
        let query = dns_socket
            .start_query(iface.context(), name, DnsQueryType::A)
            .unwrap();

        (dns_handle, query)
    };

    loop {
        let mut sockets = SOCKETS.lock().unwrap();
        let timestamp = iface_poll(&mut iface, &mut sockets);

        let dns_socket = sockets.get_mut::<dns::Socket>(dns_handle);
        match dns_socket.get_query_result(query) {
            Ok(addrs) => {
                let addr = match addrs.get(0).unwrap() {
                    smoltcp::wire::IpAddress::Ipv4(ipv4) => ipv4.0,
                };
                sockets.remove(dns_handle);
                return Ok(Ipv4Addr::from(addr));
            }
            Err(GetQueryResultError::Pending) => {} // not done yet
            Err(e) => panic!("query failed: {e:?}"),
        }

        try_phy_wait(timestamp, &mut iface, &mut sockets).expect("wait err")
    }
}

#[no_mangle]
pub fn smol_connect(sockaddr: SocketAddrV4) -> Result<SockFd, ()> {
    let mut iface = IFACE.lock().unwrap();
    let mut sockets = SOCKETS.lock().unwrap();

    let tcp_handle = {
        let tcp_rx_buffer = tcp::SocketBuffer::new(vec![0; 1024]);
        let tcp_tx_buffer = tcp::SocketBuffer::new(vec![0; 1024]);
        let socket = tcp::Socket::new(tcp_rx_buffer, tcp_tx_buffer);
        sockets.add(socket)
    };

    // let timestamp = Instant::now();
    // iface.poll(timestamp, device, &mut sockets);

    let tcp_socket: &mut tcp::Socket = sockets.get_mut::<tcp::Socket>(tcp_handle);
    if tcp_socket.is_active() {
        // bad state.
        return Err(());
    };

    let cx = iface.context();
    let local_port = 49152 + rand::random::<u16>() % 16384;
    tcp_socket.connect(cx, sockaddr, local_port).unwrap();

    Ok(to_sockfd(tcp_handle))
}

#[no_mangle]
pub fn smol_send(handle: SockFd, data: &[u8]) -> Result<(), ()> {
    let mut iface = IFACE.lock().unwrap();
    let mut sockets = SOCKETS.lock().unwrap();

    loop {
        let timestamp = iface_poll(&mut iface, &mut sockets);

        let socket = sockets.get_mut::<tcp::Socket>(from_sockfd(handle));

        if socket.may_send() {
            socket.send_slice(data).expect("cannot send");
            return Ok(());
        }

        try_phy_wait(timestamp, &mut iface, &mut sockets).expect("wait error")
    }
}

#[no_mangle]
pub fn smol_recv(handle: SockFd, buf: &mut [u8]) -> Result<Size, ()> {
    let mut iface = IFACE.lock().unwrap();
    let mut sockets = SOCKETS.lock().unwrap();

    let mut cursor = 0;
    let mut freesize = buf.len();
    loop {
        let timestamp = iface_poll(&mut iface, &mut sockets);

        let tcp_socket = sockets.get_mut::<tcp::Socket>(from_sockfd(handle));

        if freesize == 0 || !tcp_socket.may_recv() {
            return Ok(cursor);
        } else if tcp_socket.can_recv() {
            tcp_socket
                .recv(|data| {
                    // avoid to overflow the buffer.
                    let size = min(buf.len() - cursor, data.len());
                    buf[cursor..(cursor + size)].copy_from_slice(&data[0..size]);
                    cursor += size;
                    freesize = buf.len() - cursor;
                    // println!("read size={}, free size={}", size, freesize);
                    (size, ())
                })
                .expect("recv to slice failed");
        };

        try_phy_wait(timestamp, &mut iface, &mut sockets).expect("wait error");
    }
}

#[no_mangle]
pub fn smol_close() {}

#[no_mangle]
pub fn smol_bind(addr: SocketAddrV4) -> LibOSResult<SockFd> {
    let mut iface = IFACE.lock().unwrap();
    let mut sockets = SOCKETS.lock().unwrap();

    let tcp_handle = {
        let tcp_rx_buffer = tcp::SocketBuffer::new(vec![0; 64]);
        let tcp_tx_buffer = tcp::SocketBuffer::new(vec![0; 128]);
        let socket = tcp::Socket::new(tcp_rx_buffer, tcp_tx_buffer);
        sockets.add(socket)
    };

    iface_poll(&mut iface, &mut sockets);

    let socket = sockets.get_mut::<tcp::Socket>(tcp_handle);
    if socket.is_open() {
        return Err(LibOSErr::WrongSockState);
    };
    if socket.listen(addr.port()).is_err() {
        return Err(LibOSErr::TcpListenErr);
    }

    Ok(to_sockfd(tcp_handle))
}

#[no_mangle]
pub fn smol_accept(handle: SockFd) -> LibOSResult<SockFd> {
    let mut iface = IFACE.lock().unwrap();
    let mut sockets = SOCKETS.lock().unwrap();

    let conned_sock = loop {
        let timestamp = iface_poll(&mut iface, &mut sockets);
        let listened_sock = sockets.get_mut::<tcp::Socket>(from_sockfd(handle));

        println!("{:?}", listened_sock.state());
        if listened_sock.state() != State::Listen {
            break listened_sock;
        }

        if try_phy_wait(timestamp, &mut iface, &mut sockets).is_err() {
            return Err(LibOSErr::PhyWaitErr);
        }
    };

    // println!("sock state is {:?}", sock.state());
    let local = conned_sock.local_endpoint().unwrap().port;

    drop(iface);
    drop(sockets);

    smol_bind(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), local))
}
