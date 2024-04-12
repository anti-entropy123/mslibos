use std::{
    fs,
    io::{Seek, SeekFrom, Write},
    mem::{self, transmute},
    num::NonZeroUsize,
    os::{
        fd::{AsRawFd, OwnedFd},
        raw::c_void,
    },
    process,
    ptr::read_volatile,
    slice,
    thread::sleep,
    time::{Duration, SystemTime},
};

use nix::{
    libc::{self, SIGCHLD},
    sys::mman::{mmap, MapFlags, ProtFlags},
    unistd::{pipe, read, write},
};

use crate::{DataReceiver, DataSender, TransferData};

type CloneCb = Box<dyn FnMut() -> i32>;

fn clone3(cb: &mut CloneCb, flags: u64, exit_signal: Option<u64>) -> Result<i32, String> {
    #[repr(C)]
    struct Clone3Args {
        flags: u64,
        pidfd: u64,
        child_tid: u64,
        parent_tid: u64,
        exit_signal: u64,
        stack: u64,
        stack_size: u64,
        tls: u64,
        set_tid: u64,
        set_tid_size: u64,
        cgroup: u64,
    }
    let mut args = Clone3Args {
        flags,
        pidfd: 0,
        child_tid: 0,
        parent_tid: 0,
        exit_signal: exit_signal.unwrap_or(0),
        stack: 0,
        stack_size: 0,
        tls: 0,
        set_tid: 0,
        set_tid_size: 0,
        cgroup: 0,
    };
    let args_ptr = &mut args as *mut Clone3Args;
    let args_size = std::mem::size_of::<Clone3Args>();
    match unsafe { libc::syscall(libc::SYS_clone3, args_ptr, args_size) } {
        -1 => Err(nix::Error::last().to_string()),
        0 => {
            std::process::exit(cb());
        }
        ret if ret >= 0 => Ok(ret as i32),
        _ret => Err("unknown errno".to_owned()),
    }
}

pub struct ShareMem;

impl TransferData for ShareMem {
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

        let mut child_proc: CloneCb = {
            Box::new(move || {
                Self::child_proc(
                    &file_path,
                    data_size,
                    mmap_length,
                    channel_reciver.try_clone().unwrap(),
                    ack_writer.try_clone().unwrap(),
                )
            })
        };
        clone3(&mut child_proc, 0, Some(SIGCHLD as u64)).unwrap();

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
}

impl ShareMem {
    fn child_proc(
        file_path: &String,
        data_size: usize,
        mmap_length: NonZeroUsize,
        chan_receiver: OwnedFd,
        ack_writer: OwnedFd,
    ) -> i32 {
        let ram_file = fs::File::open(file_path).unwrap();
        let buffer = unsafe {
            mmap(
                None,
                mmap_length,
                ProtFlags::PROT_READ,
                MapFlags::MAP_SHARED,
                ram_file,
                0,
            )
            .unwrap()
            .as_ptr()
        };
        let buffer: &[u8] = unsafe { slice::from_raw_parts(transmute(buffer), data_size) };
        read(chan_receiver.as_raw_fd(), &mut [0, 1]).unwrap();

        for item in buffer {
            let _: u8 = unsafe { read_volatile(item as *const u8) };
        }
        let time_stamp = SystemTime::now();
        unsafe {
            let time_stamp = slice::from_raw_parts(
                transmute(&time_stamp as *const _),
                mem::size_of::<SystemTime>(),
            );
            write(ack_writer, time_stamp).unwrap();
        };

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
            *item = idx as u8;
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
    fn receive(&self) -> SystemTime {
        let mut buffer = [0u8; 100];
        read(self.ack_reader.as_raw_fd(), &mut buffer).unwrap();
        let ok_time: &SystemTime = unsafe { transmute(&buffer) };
        *ok_time
    }
}
