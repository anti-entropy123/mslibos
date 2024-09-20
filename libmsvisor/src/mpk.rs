use std::{arch::asm, ffi::c_void};

use nix::{
    errno::Errno,
    libc::{size_t, syscall, SYS_pkey_alloc, SYS_pkey_mprotect},
};

pub fn pkey_alloc() -> i32 {
    unsafe { syscall(SYS_pkey_alloc, 0, 0) as i32 }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn pkey_mprotect(addr: *mut c_void, len: size_t, prot: i32, pkey: i32) -> Result<(), i32> {
    let ret: i64;
    unsafe {
        ret = syscall(SYS_pkey_mprotect, addr, len, prot, pkey);
    }
    match ret {
        0 => Ok(()),
        -1 => Err(Errno::last_raw()),
        _ => Err(Errno::last_raw()),
    }
}

pub fn pkey_read() -> u32 {
    // Reads the value of PKRU into EAX and clears EDX. ECX must be 0 when RDPKRU is executed; otherwise, a general-protection exception (#GP) occurs.
    let result: u32;
    // unsafe { asm!("rdpkru":  "=eax"(result) : "ecx"(0) : "edx": "volatile") }
    unsafe {
        asm!(
            "rdpkru",
            out("eax") result,
            in("ecx") 0,
            lateout("edx") _
        )
    };
    result
}

#[inline]
pub fn pkey_write(pkru: u32) {
    // Writes the value of EAX into PKRU. ECX and EDX must be 0 when WRPKRU is executed;
    // otherwise, a general-protection exception (#GP) occurs.
    let eax = pkru;
    let ecx = 0;
    let edx = 0;

    unsafe {
        asm!(
            "wrpkru",
            in("eax") eax,
            in("ecx") ecx,
            in("edx") edx,
        )
    };
}

#[inline]
pub fn pkey_set(pkey: i32, rights: u32) -> Result<(), &'static str> {
    if (0..16).contains(&pkey) {
        let mask: u32 = 0b11 << (2 * pkey);
        let mut pkru = pkey_read();
        pkru = (pkru & !mask) | (rights << (2 * pkey));
        pkey_write(pkru);
        Ok(())
    } else {
        Err("Invalid PKEY")
    }
}

#[test]
fn test_mpk() {
    let pkey1 = pkey_alloc();
    println!("get pkey: {}", pkey1);
    let mm_layout = Layout::from_size_align(4096 * 10, 4096).unwrap();
    let mm_region = unsafe { alloc::alloc(mm_layout) };
    pkey_mprotect(
        mm_region.cast(),
        4096 * 10,
        libc::PROT_READ | libc::PROT_WRITE,
        pkey1,
    )
    .unwrap();
    let pkru = pkey_read();
    println!("pkru: {:b}", pkru);
    // 没有关闭 pkey1 的权限, 所以可以访问 group1 的内存
    let data = unsafe { slice::from_raw_parts_mut(mm_region, 4096 * 10) };
    data[0] = 100;
    println!("data: {}", data[0]);

    // 关闭读写权限.
    pkey_set(pkey1, 3).unwrap();
    let pkru = pkey_read();
    println!("pkru: {:b}", pkru);

    // 下一行预期会报错:
    println!("data: {}", data[0]);
}
