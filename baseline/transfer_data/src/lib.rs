use std::{
    mem::{self, transmute},
    os::fd::{AsRawFd, OwnedFd},
    ptr::read_volatile,
    slice,
    time::SystemTime,
};

use nix::unistd::read;

pub mod fork;
pub mod share_mem;
pub mod socket;

use fork::*;

pub trait TransferData
where
    Self::ChildArgs: Clone,
{
    type ChildArgs;
    fn build(data_size: usize) -> (Box<dyn DataSender>, Box<dyn DataReceiver>);
    fn child_proc(args: Self::ChildArgs) -> i32;

    fn access_data(buffer: &[u8]) {
        for item in buffer {
            unsafe { read_volatile(item as *const u8) };
        }
    }

    fn ack_with_time(ack_writer: OwnedFd) {
        let time_stamp = SystemTime::now();
        unsafe {
            let time_stamp = slice::from_raw_parts(
                transmute(&time_stamp as *const _),
                mem::size_of::<SystemTime>(),
            );
            nix::unistd::write(ack_writer, time_stamp).unwrap();
        };
    }
}

pub trait DataSender {
    fn send(&self) -> SystemTime;
}

pub trait DataReceiver {
    fn ack_reader(&self) -> &OwnedFd;

    fn receive(&self) -> SystemTime {
        let mut buffer = [0u8; 100];

        read(self.ack_reader().as_raw_fd(), &mut buffer).unwrap();
        let ok_time: &SystemTime = unsafe { transmute(&buffer) };
        *ok_time
    }
}
