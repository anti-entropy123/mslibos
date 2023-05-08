#![feature(ip_in_core)]

use core::net::Ipv4Addr;
use std::os::fd::AsRawFd;

use ms_hostcall::types::{NetDevice, NetIface};
use smoltcp::{
    iface::{Config, Interface, SocketSet},
    phy::{wait as phy_wait, Device, Medium, TunTapInterface},
    socket::dns::{self, GetQueryResultError},
    time::Instant,
    wire::{DnsQueryType, EthernetAddress, Ipv4Address, IpCidr, IpAddress},
};

#[no_mangle]
pub fn init_net_dev() -> (NetDevice, NetIface) {
    // Create device;
    let mut device: TunTapInterface = TunTapInterface::new("tap0", Medium::Ethernet).unwrap();

    // Create interface
    let mut iface = {
        let mut config = match device.capabilities().medium {
            Medium::Ethernet => {
                Config::new(EthernetAddress([0x02, 0x00, 0x00, 0x00, 0x00, 0x01]).into())
            }
            Medium::Ip => Config::new(smoltcp::wire::HardwareAddress::Ip),
            Medium::Ieee802154 => todo!(),
        };
        config.random_seed = rand::random();
        Interface::new(config, &mut device)
    };

    iface.update_ip_addrs(|ip_addrs| {
        ip_addrs
            .push(IpCidr::new(IpAddress::v4(192, 168, 69, 1), 24))
            .unwrap();
    });
    iface
        .routes_mut()
        .add_default_ipv4_route(Ipv4Address::new(192, 168, 69, 100))
        .unwrap();

    (device, iface)
}

#[no_mangle]
pub fn addrinfo(device: &mut NetDevice, iface: &mut NetIface, name: &str) -> Result<Ipv4Addr, ()> {
    let fd = device.as_raw_fd();
    println!("device raw fd: {}", fd);
    // Create sockets
    let servers = &[Ipv4Address::new(8, 8, 8, 8).into()];
    let dns_socket = dns::Socket::new(servers, vec![]);

    let mut sockets = SocketSet::new(vec![]);
    let dns_handle = sockets.add(dns_socket);

    let socket = sockets.get_mut::<dns::Socket>(dns_handle);
    let query: dns::QueryHandle = socket
        .start_query(iface.context(), name, DnsQueryType::A)
        .unwrap();

    loop {
        let timestamp = Instant::now();
        //  log::debug!("timestamp {:?}", timestamp);

        iface.poll(timestamp, device, &mut sockets);

        match sockets
            .get_mut::<dns::Socket>(dns_handle)
            .get_query_result(query)
        {
            Ok(addrs) => {
                let addr = match addrs.get(0).unwrap() {
                    smoltcp::wire::IpAddress::Ipv4(ipv4) => ipv4.0,
                    smoltcp::wire::IpAddress::Ipv6(_) => todo!(),
                };
                return Ok(Ipv4Addr::from(addr));
            }
            Err(GetQueryResultError::Pending) => {} // not done yet
            Err(e) => panic!("query failed: {e:?}"),
        }

        phy_wait(fd, iface.poll_delay(timestamp, &sockets)).expect("wait error");
    }

}

#[no_mangle]
pub fn connect() {}

#[no_mangle]
pub fn send() {}

#[no_mangle]
pub fn recv() {}

#[no_mangle]
pub fn close_tcp() {}
