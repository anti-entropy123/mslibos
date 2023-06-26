#![feature(ip_in_core)]
#![allow(clippy::result_unit_err)]

mod drop;
mod setup_tap;

use core::net::Ipv4Addr;
use std::{cmp::min, net::SocketAddrV4, os::fd::AsRawFd, sync::Mutex};

use lazy_static::lazy_static;
use smoltcp::{
    iface::{Config, Interface, SocketHandle, SocketSet},
    phy::{wait as phy_wait, Device, Medium, TunTapInterface},
    socket::{
        dns::{self, GetQueryResultError},
        tcp,
    },
    time::Instant,
    wire::{DnsQueryType, EthernetAddress, IpAddress, IpCidr, Ipv4Address},
};

use crate::setup_tap::exec_tap_setup;
use ms_hostcall::types::NetdevName;
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
}

lazy_static! {
    /// NETDEV_NAME use Option<> because the `socket` module may never be used so in
    /// this case shouldn't ask for a NetdevName.
    pub static ref NETDEV_NAME: Mutex<Option<NetdevName>> = Mutex::default();
    static ref SOCKETS: Mutex<SocketSet<'static>> = Mutex::new(SocketSet::new(vec![]));
    static ref IFACE: Mutex<Interface> = {
        let mut iface = DEVICE.with(|device_tls| {
            let mut device = device_tls.lock().unwrap();
            let mut config = Config::new();
            match device.capabilities().medium {
                Medium::Ethernet => {
                    config.hardware_addr =
                        Some(EthernetAddress([0x02, 0x00, 0x00, 0x00, 0x00, 0x01]).into());
                }
                Medium::Ip => todo!(),
                // Medium::Ieee802154 => todo!(),
            };
            config.random_seed = rand::random();
            Interface::new(config, &mut (*device))
        });

        iface.update_ip_addrs(|ip_addrs| {
            ip_addrs
                .push(IpCidr::new(IpAddress::v4(
                                192, 168, 69,
                                init_context::isolation_ctx().isol_id as u8
                            ), 24)).unwrap();
        });
        iface
            .routes_mut()
            .add_default_ipv4_route(Ipv4Address::new(192, 168, 69, 100))
            .unwrap();

        Mutex::new(iface)
    };
}

#[no_mangle]
pub fn addrinfo(name: &str) -> Result<Ipv4Addr, ()> {
    let fd = DEVICE.with(|device| device.lock().unwrap().as_raw_fd());
    // println!("Device raw_fd is {}", fd);
    // Create sockets
    let servers = &[Ipv4Address::new(8, 8, 8, 8).into()];
    let dns_socket = dns::Socket::new(servers, vec![]);

    let mut sockets = SOCKETS.lock().unwrap();
    let dns_handle: SocketHandle = sockets.add(dns_socket);

    let socket = sockets.get_mut::<dns::Socket>(dns_handle);
    let mut iface = IFACE.lock().unwrap();

    let query: dns::QueryHandle = socket
        .start_query(iface.context(), name, DnsQueryType::A)
        .unwrap();

    loop {
        let timestamp = Instant::now();
        DEVICE.with(|device_tls| {
            let mut device = device_tls.lock().unwrap();
            iface.poll(timestamp, &mut (*device), &mut sockets)
        });

        match sockets
            .get_mut::<dns::Socket>(dns_handle)
            .get_query_result(query)
        {
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

        phy_wait(fd, iface.poll_delay(timestamp, &sockets)).expect("wait error");
    }
}

#[no_mangle]
pub fn connect(sockaddr: SocketAddrV4) -> Result<(), ()> {
    // let address = address
    // Create sockets
    let tcp_socket = {
        let tcp_rx_buffer = tcp::SocketBuffer::new(vec![0; 1024]);
        let tcp_tx_buffer = tcp::SocketBuffer::new(vec![0; 1024]);
        tcp::Socket::new(tcp_rx_buffer, tcp_tx_buffer)
    };
    let mut sockets = SOCKETS.lock().unwrap();
    let tcp_handle: SocketHandle = sockets.add(tcp_socket);

    // let timestamp = Instant::now();
    // iface.poll(timestamp, device, &mut sockets);

    let socket: &mut tcp::Socket = sockets.get_mut::<tcp::Socket>(tcp_handle);
    let mut iface = IFACE.lock().unwrap();
    let cx = iface.context();

    assert!(!socket.is_active());
    let local_port = 49152 + rand::random::<u16>() % 16384;
    socket.connect(cx, sockaddr, local_port).unwrap();

    Ok(())
}

#[no_mangle]
pub fn send(data: &[u8]) -> Result<(), ()> {
    let fd = DEVICE.with(|device| device.lock().unwrap().as_raw_fd());
    let mut iface = IFACE.lock().unwrap();

    loop {
        let timestamp = Instant::now();
        DEVICE.with(|device_tls| {
            let mut device = device_tls.lock().unwrap();
            iface.poll(timestamp, &mut (*device), &mut SOCKETS.lock().unwrap())
        });

        let mut sockets = SOCKETS.lock().unwrap();
        let socket = sockets.get_mut::<tcp::Socket>(SocketHandle::default());

        if socket.may_send() {
            socket.send_slice(data).expect("cannot send");
            return Ok(());
        }
        // if don't manual drop sockets, will be deadlock.
        drop(sockets);
        phy_wait(fd, iface.poll_delay(timestamp, &SOCKETS.lock().unwrap())).expect("wait error");
    }
}

#[no_mangle]
pub fn recv(buf: &mut [u8]) -> Result<usize, ()> {
    let fd = DEVICE.with(|device| device.lock().unwrap().as_raw_fd());
    let mut iface = IFACE.lock().unwrap();

    let mut cursor = 0;
    let mut freesize = buf.len();
    loop {
        let timestamp = Instant::now();
        DEVICE.with(|device_tls| {
            let mut device = device_tls.lock().unwrap();
            iface.poll(timestamp, &mut (*device), &mut SOCKETS.lock().unwrap())
        });

        let mut sockets = SOCKETS.lock().unwrap();
        let socket = sockets.get_mut::<tcp::Socket>(SocketHandle::default());

        if freesize == 0 || !socket.may_recv() {
            return Ok(cursor);
        } else if socket.can_recv() {
            socket
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

        // The reason for this line has been said in function `send`.
        drop(sockets);
        phy_wait(fd, iface.poll_delay(timestamp, &SOCKETS.lock().unwrap())).expect("wait error");
    }
}

#[no_mangle]
pub fn close_tcp() {}
