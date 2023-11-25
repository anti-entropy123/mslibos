extern crate alloc;

use core::alloc::Layout;

use libc::c_void;
use ms_hostcall::{
    err::{LibOSErr, LibOSResult},
    types::{Fd, ProtFlags},
};
use ms_std::libos::libos;

const PAGE_SIZE: usize = 0x1000;

#[no_mangle]
pub fn libos_mmap(length: usize, prot: ProtFlags, fd: Fd) -> LibOSResult<usize> {
    if length % PAGE_SIZE > 0 {
        return Err(LibOSErr::BadArgs);
    }
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

    // println!("finish mmap, mmap_addr={}", mmap_addr);
    Ok(mmap_addr)
}

#[no_mangle]
pub fn libos_munmap(mem_region: &mut [u8], _file_based: bool) -> LibOSResult<()> {
    libos!(unregister_file_backend(mem_region.as_ptr() as usize))
        .expect("unregister file backend failed.");

    let aligned_length = (mem_region.len() + PAGE_SIZE - 1) & (!PAGE_SIZE + 1);
    unsafe {
        libc::mprotect(
            mem_region.as_mut_ptr() as usize as *mut libc::c_void,
            aligned_length,
            libc::PROT_READ | libc::PROT_WRITE,
        );
        alloc::alloc::dealloc(
            mem_region.as_mut_ptr(),
            Layout::from_size_align(aligned_length, PAGE_SIZE).expect("wrong align."),
        );
    };

    Ok(())
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
