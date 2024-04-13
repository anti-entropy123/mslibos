use std::{
    io::{Read, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    os::fd::OwnedFd,
    thread::sleep,
    time::{Duration, SystemTime},
};

use nix::unistd::pipe;

use crate::{clone3, CloneCb, DataReceiver, DataSender, TransferData};

const PORT_NUM: u16 = 15678;

pub struct ChildArgs {
    data_size: usize,
    ack_writer: OwnedFd,
}

impl Clone for ChildArgs {
    fn clone(&self) -> Self {
        Self {
            data_size: self.data_size,
            ack_writer: self.ack_writer.try_clone().unwrap(),
        }
    }
}

pub struct TcpSocket;

impl TransferData for TcpSocket {
    type ChildArgs = ChildArgs;

    fn build(data_size: usize) -> (Box<dyn crate::DataSender>, Box<dyn crate::DataReceiver>) {
        let (ack_reader, ack_writer) = pipe().unwrap();

        let args = Self::ChildArgs {
            data_size,
            ack_writer: ack_writer.try_clone().unwrap(),
        };
        let child_proc: CloneCb = { Box::new(move || Self::child_proc(args.clone())) };
        clone3(child_proc, 0).unwrap();

        (
            Box::new(TcpSockerSender { data_size }),
            Box::new(TcpSocketReceiver { ack_reader }),
        )
    }

    fn child_proc(args: Self::ChildArgs) -> i32 {
        let listener = TcpListener::bind(SocketAddr::new(
            std::net::IpAddr::V4(Ipv4Addr::LOCALHOST),
            PORT_NUM,
        ))
        .unwrap();

        let mut conn = listener.accept().unwrap();
        let mut buffer = Vec::with_capacity(args.data_size);
        conn.0.read_to_end(&mut buffer).unwrap();

        Self::access_data(&buffer);
        Self::ack_with_time(args.ack_writer);

        0
    }
}

struct TcpSockerSender {
    data_size: usize,
}

impl DataSender for TcpSockerSender {
    fn send(&self) -> std::time::SystemTime {
        let mut data: Vec<u8> = Vec::with_capacity(self.data_size);
        for i in 0..self.data_size {
            data.push((i % (u8::MAX as usize - 1)) as u8)
        }

        sleep(Duration::from_millis(100));
        let now = SystemTime::now();
        let mut conn =
            TcpStream::connect(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), PORT_NUM)).unwrap();
        conn.write_all(&data).unwrap();

        now
    }
}

struct TcpSocketReceiver {
    ack_reader: OwnedFd,
}
impl DataReceiver for TcpSocketReceiver {
    fn ack_reader(&self) -> &OwnedFd {
        &self.ack_reader
    }
}
