extern crate alloc;

use core::alloc::Layout;

use libc::c_void;
use ms_hostcall::{
    err::{LibOSErr, LibOSResult},
    types::{Fd, ProtFlags},
};
use ms_std::libos::libos;

#[no_mangle]
pub fn mmap(length: usize, prot: ProtFlags, fd: Fd) -> LibOSResult<usize> {
    let layout = Layout::from_size_align(length, 0x1000).map_err(|_| LibOSErr::BadArgs)?;

    let mmap_addr = unsafe {
        let addr = alloc::alloc::alloc(layout) as usize as *mut libc::c_void;
        assert_eq!(libc::munmap(addr, length), 0, "munmap failed");
        assert_eq!(
            libc::mmap(
                addr,
                length,
                trans_protflag(prot),
                libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
                0,
                0,
            ),
            addr,
            "mmap failed"
        );

        addr as usize
    };

    let mm_region = unsafe { core::slice::from_raw_parts_mut(mmap_addr as *mut c_void, length) };
    libos!(register_file_backend(mm_region, fd)).expect("register_file_backend failed.");

    Ok(0)
}

pub fn trans_protflag(flags: ProtFlags) -> i32 {
    let mut result = Default::default();
    if flags.contains(ProtFlags::READ) {
        result |= libc::PROT_READ
    }
    if flags.contains(ProtFlags::WRITE) {
        result |= libc::PROT_WRITE
    }
    if flags.contains(ProtFlags::EXEC) {
        unimplemented!("ProtFlags::EXEC")
    }

    result
}
