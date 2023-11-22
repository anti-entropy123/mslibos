use core::{mem::ManuallyDrop, slice};

use ms_hostcall::types::ProtFlags;

use crate::{fs::File, libos::libos};

pub struct Mmap {
    ptr: usize,
    length: usize,
}

impl Mmap {
    pub fn mmap_file(file: File) -> Result<Self, ()> {
        let file = ManuallyDrop::new(file);
        let length = 0x1000;
        let ptr =
            libos!(mmap(length, ProtFlags::READ, file.as_raw_fd())).expect("libos mmap failed");

        Ok(Mmap { ptr, length })
    }
}

impl AsRef<[u8]> for Mmap {
    fn as_ref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr as *const u8, self.length) }
    }
}
