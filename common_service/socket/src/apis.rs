use core::{
    cmp::min,
    net::{Ipv4Addr, SocketAddrV4},
};

use ms_hostcall::{
    err::{LibOSErr, LibOSResult},
    types::{Size, SockFd},
};
use smoltcp::{
    iface::SocketHandle,
    socket::{
        dns::{self, GetQueryResultError},
        tcp::{self, State},
    },
    wire::{DnsQueryType, Ipv4Address},
};

use crate::{from_sockfd, iface_poll, to_sockfd, try_phy_wait, IFACE, SOCKETS};

#[no_mangle]
pub fn addrinfo(name: &str) -> Result<Ipv4Addr, ()> {
    // println!("Device raw_fd is {}", fd);

    let mut iface = IFACE.lock().unwrap();

    let (dns_handle, query) = {
        let servers = &[Ipv4Address::new(114, 114, 114, 114).into()];
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
    // println!("smol_connect");
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
    // println!("smol_send, handle={}", handle);
    let mut iface = IFACE.lock().unwrap();
    let mut sockets = SOCKETS.lock().unwrap();
    let mut cursor = 0;

    loop {
        let timestamp = iface_poll(&mut iface, &mut sockets);

        let socket = sockets.get_mut::<tcp::Socket>(from_sockfd(handle));

        // send_slice doesn't mean it's actually sent
        if socket.may_send() {
            // println!(
            //     "sock_{} remote={:?}, send slice: {:?},",
            //     handle,
            //     socket.remote_endpoint(),
            //     String::from_utf8_lossy(&data[cursor..])
            // );
            cursor += socket.send_slice(&data[cursor..]).expect("cannot send");
        }

        if cursor == data.len() {
            return Ok(());
        }

        try_phy_wait(timestamp, &mut iface, &mut sockets).expect("wait error")
    }
}

#[no_mangle]
pub fn smol_recv(handle: SockFd, buf: &mut [u8]) -> Result<Size, ()> {
    // println!("smol_recv");
    let mut iface = IFACE.lock().unwrap();
    let mut sockets = SOCKETS.lock().unwrap();

    let mut cursor = 0;
    let mut freesize = buf.len();

    // should return data if buffer is not empty. otherwise will block until closed TCP.
    loop {
        let timestamp = iface_poll(&mut iface, &mut sockets);
        let tcp_socket = sockets.get_mut::<tcp::Socket>(from_sockfd(handle));

        // println!("wait for recv, status={}", tcp_socket.state());
        if cursor != 0 || !tcp_socket.may_recv() || tcp_socket.state() == State::CloseWait {
            break;
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

    Ok(cursor)
}

#[no_mangle]
pub fn smol_close(handle: SockFd) -> LibOSResult<()> {
    // println!("smol_close");
    let mut iface = IFACE.lock().unwrap();

    let handle = from_sockfd(handle);
    let mut has_close = false;

    loop {
        let mut sockets = SOCKETS.lock().unwrap();
        let timestamp = iface_poll(&mut iface, &mut sockets);

        let tcp_socket = sockets.get_mut::<tcp::Socket>(handle);
        if !has_close && tcp_socket.send_queue() == 0 {
            // println!(
            //     "send queue is empty, can close sock: {}, state: {}",
            //     handle,
            //     tcp_socket.state()
            // );

            // Should not remove socket from sockets. Because smoltcp will
            // update state of tcp connect by sockets.
            tcp_socket.close();
            has_close = true;
        }

        // println!("{:?}", tcp_socket.state());
        if !tcp_socket.is_open() {
            break;
        }

        if try_phy_wait(timestamp, &mut iface, &mut sockets).is_err() {
            return Err(LibOSErr::PhyWaitErr);
        };
    }

    Ok(())
}

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

        // println!("{:?}", listened_sock.state());
        if listened_sock.state() != State::Listen {
            break listened_sock;
        }

        if try_phy_wait(timestamp, &mut iface, &mut sockets).is_err() {
            return Err(LibOSErr::PhyWaitErr);
        }
    };

    // println!("sock state is {:?}", sock.state());
    // println!("remote is {}", conned_sock.remote_endpoint().unwrap());
    let local = conned_sock.local_endpoint().unwrap().port;

    drop(iface);
    drop(sockets);

    smol_bind(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), local))
}
