use std::{mem::transmute, num::NonZeroUsize, ptr::read_volatile, slice, time::SystemTime};

use nix::sys::mman::{mmap_anonymous, MapFlags, ProtFlags};

use crate::{DataReceiver, DataSender, TransferData};

pub struct Function;

impl TransferData for Function {
    type ChildArgs = ();

    fn build(data_size: usize) -> (Box<dyn crate::DataSender>, Box<dyn crate::DataReceiver>) {
        let mmap_length =
            NonZeroUsize::new((data_size as f64 / 4096.0).ceil() as usize * 4096).unwrap();
        let buffer = unsafe {
            mmap_anonymous(
                None,
                mmap_length,
                ProtFlags::PROT_WRITE | ProtFlags::PROT_READ,
                MapFlags::MAP_PRIVATE | MapFlags::MAP_ANONYMOUS,
            )
            .unwrap()
            .as_ptr()
        };

        (
            Box::new(FunctionSender {
                data_size,
                base_addr: unsafe { transmute(buffer) },
            }),
            Box::new(FunctionReceiver {
                data_size,
                base_addr: unsafe { transmute(buffer) },
            }),
        )
    }

    fn child_proc(_args: Self::ChildArgs) -> i32 {
        unreachable!()
    }
}

pub struct FunctionSender {
    data_size: usize,
    base_addr: *const u8,
}

impl DataSender for FunctionSender {
    fn send(&self) -> std::time::SystemTime {
        let buffer: &mut [u8] =
            unsafe { slice::from_raw_parts_mut(transmute(self.base_addr), self.data_size) };

        for (idx, item) in buffer.iter_mut().enumerate() {
            *item = (idx % (u8::MAX - 1) as usize) as u8;
        }

        SystemTime::now()
    }
}

pub struct FunctionReceiver {
    data_size: usize,
    base_addr: *const u8,
}

impl DataReceiver for FunctionReceiver {
    fn ack_reader(&self) -> &std::os::unix::prelude::OwnedFd {
        unreachable!()
    }

    fn receive(&self) -> std::time::SystemTime {
        let buffer = unsafe { slice::from_raw_parts(self.base_addr, self.data_size) };
        for item in buffer {
            unsafe { read_volatile(item as *const u8) };
        }
        SystemTime::now()
    }
}
