#![feature(ip_in_core)]

use core::net::Ipv4Addr;
use std::{cmp::min, net::SocketAddrV4, os::fd::AsRawFd, sync::Mutex};

use lazy_static::lazy_static;
use smoltcp::{
    iface::{Config, Interface, SocketHandle, SocketSet},
    phy::{wait as phy_wait, Device, Medium, TunTapInterface},
    socket::{
        dns::{self, GetQueryResultError},
        tcp::{self},
    },
    time::Instant,
    wire::{DnsQueryType, EthernetAddress, IpAddress, IpCidr, Ipv4Address},
};

thread_local! {
    static DEVICE: Mutex<TunTapInterface> = Mutex::from(TunTapInterface::new("tap0", Medium::Ethernet).unwrap()) ;
}

lazy_static! {
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
                .push(IpCidr::new(IpAddress::v4(192, 168, 69, 1), 24))
                .unwrap();
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
