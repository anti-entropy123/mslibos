use core::{mem::ManuallyDrop, slice};

use ms_hostcall::types::ProtFlags;

use crate::{fs::File, libos::libos};

const PAGE_SIZE: usize = 0x1000;

pub struct Mmap {
    ptr: usize,
    length: usize,
}

impl Mmap {
    pub fn mmap_file(file: File) -> Result<Self, ()> {
        let file = ManuallyDrop::new(file);
        let length = file.metadata().expect("file stat failed.").st_size;
        let aligned_length = (length + PAGE_SIZE - 1) & (!PAGE_SIZE + 1);
        let ptr = libos!(mmap(aligned_length, ProtFlags::READ, file.as_raw_fd()))
            .expect("libos mmap failed");

        Ok(Mmap { ptr, length })
    }
}

impl AsRef<[u8]> for Mmap {
    fn as_ref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr as *const u8, self.length) }
    }
}

impl Drop for Mmap {
    fn drop(&mut self) {
        let region = unsafe { slice::from_raw_parts_mut(self.ptr as *mut u8, self.length) };
        libos!(munmap(region)).expect("munmap failed")
    }
}
