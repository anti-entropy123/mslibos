use std::{
    fs,
    io::{Seek, SeekFrom, Write},
    mem::transmute,
    num::NonZeroUsize,
    os::{
        fd::{AsRawFd, OwnedFd},
        raw::c_void,
    },
    process, slice,
    thread::sleep,
    time::{Duration, SystemTime},
};

use nix::{
    sys::mman::{mmap, MapFlags, ProtFlags},
    unistd::{pipe, read, write},
};

use crate::{clone3, CloneCb, DataReceiver, DataSender, TransferData};

pub struct ChildArgs {
    file_path: String,
    data_size: usize,
    mmap_length: NonZeroUsize,
    chan_receiver: OwnedFd,
    ack_writer: OwnedFd,
}

impl Clone for ChildArgs {
    fn clone(&self) -> Self {
        Self {
            file_path: self.file_path.clone(),
            data_size: self.data_size,
            mmap_length: self.mmap_length,
            chan_receiver: self.chan_receiver.try_clone().unwrap(),
            ack_writer: self.ack_writer.try_clone().unwrap(),
        }
    }
}

pub struct ShareMem;

impl TransferData for ShareMem {
    type ChildArgs = ChildArgs;

    fn build(data_size: usize) -> (Box<dyn DataSender>, Box<dyn DataReceiver>) {
        let mmap_length =
            NonZeroUsize::new((data_size as f64 / 4096.0).ceil() as usize * 4096).unwrap();

        let (channel_reciver, channel_sender) = pipe().unwrap();
        // (read, write)
        let (ack_reader, ack_writer) = pipe().unwrap();

        let file_path = format!("/mnt/ramfs/{}", process::id());
        let mut ram_file = fs::File::options()
            .create(true)
            .read(true)
            .write(true)
            .open(file_path.clone())
            .unwrap();
        ram_file
            .seek(SeekFrom::Start(data_size as u64 - 1))
            .unwrap();
        ram_file.write_all(&[1]).unwrap();
        ram_file.seek(SeekFrom::Start(0)).unwrap();

        let args = Self::ChildArgs {
            file_path: file_path.clone(),
            data_size,
            mmap_length,
            chan_receiver: channel_reciver.try_clone().unwrap(),
            ack_writer: ack_writer.try_clone().unwrap(),
        };
        let child_proc: CloneCb = { Box::new(move || Self::child_proc(args.clone())) };
        clone3(child_proc, 0).unwrap();

        let buffer = unsafe {
            mmap(
                None,
                mmap_length,
                ProtFlags::PROT_WRITE | ProtFlags::PROT_READ,
                MapFlags::MAP_SHARED,
                ram_file,
                0,
            )
            .unwrap()
            .as_ptr()
        };

        (
            Box::new(ShareMemSender {
                chan_sender: channel_sender,
                buffer,
                data_size,
            }),
            Box::new(ShareMemReceiver { ack_reader }),
        )
    }

    fn child_proc(args: Self::ChildArgs) -> i32 {
        let ram_file = fs::File::open(args.file_path).unwrap();
        let buffer = unsafe {
            mmap(
                None,
                args.mmap_length,
                ProtFlags::PROT_READ,
                MapFlags::MAP_SHARED,
                ram_file,
                0,
            )
            .unwrap()
            .as_ptr()
        };
        let buffer: &[u8] = unsafe { slice::from_raw_parts(transmute(buffer), args.data_size) };
        read(args.chan_receiver.as_raw_fd(), &mut [0, 1]).unwrap();

        Self::access_data(buffer);
        Self::ack_with_time(args.ack_writer);
        0
    }
}

pub struct ShareMemSender {
    chan_sender: OwnedFd,
    buffer: *mut c_void,
    data_size: usize,
}

impl DataSender for ShareMemSender {
    fn send(&self) -> SystemTime {
        let buffer: &mut [u8] =
            unsafe { slice::from_raw_parts_mut(transmute(self.buffer), self.data_size) };

        for (idx, item) in buffer.iter_mut().enumerate() {
            *item = (idx % (u8::MAX - 1) as usize) as u8;
        }
        sleep(Duration::from_millis(100));
        let start = SystemTime::now();
        write(self.chan_sender.try_clone().unwrap(), &[0; 1]).unwrap();

        start
    }
}

pub struct ShareMemReceiver {
    ack_reader: OwnedFd,
}

impl DataReceiver for ShareMemReceiver {
    fn ack_reader(&self) -> &OwnedFd {
        &self.ack_reader
    }
}
