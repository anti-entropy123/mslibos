use core::slice;

use ms_hostcall::types::{OpenFlags, OpenMode};
use ms_std::{libos::libos, println};
use tinywasm::{FuncContext, MemoryStringExt};

#[repr(C)]
struct WasiCiovec {
    buf: u32,
    buf_len: u32,
}

#[repr(C)]
struct WasiFdstat {
    fs_filetype: u8,
    fs_flags: u16,
    fs_rights_base: u64,
    fs_rights_inheriting: u64,
}

#[repr(C)]
struct WasiPrestatDir {
    dirname_len: u64,
}

#[repr(C)]
struct WasiPrestatUt {
    dir: WasiPrestatDir,
}

#[repr(C)]
struct WasiPrestatT {
    tag: u8,
    u: WasiPrestatUt,
}

pub fn fd_close(mut _ctx: FuncContext, _args: i32) -> tinywasm::Result<i32> {
    println!("[Debug] Invoke into fd_close\n");

    let fd = _args as u32;
    libos!(close(fd)).unwrap();

    Ok(0)
}

pub fn fd_fdstat_get(mut ctx: FuncContext<'_>, args: (i32, i32)) -> tinywasm::Result<i32> {
    println!("[Debug] Invoke into fd_fdstat_get\n");
    println!("args: {:?}, {:?}", args.0, args.1);

    let fd = args.0 as u32;
    let addr = args.1 as usize;
    let mut mem = ctx.exported_memory_mut("memory")?;

    /* fake implementation */

    let mut fdstat = WasiFdstat {
        fs_filetype: 0b00010000,
        fs_flags: 0b00000,
        fs_rights_base: 0xFFFFFFFFFFFFFFFF,
        fs_rights_inheriting: 0xFFFFFFFFFFFFFFFF,
    };

    if fd == 1 || fd == 2 {
        fdstat.fs_flags = 1;
        fdstat.fs_filetype = 0b00000100;
    }

    let ret = (&fdstat) as *const _ as usize;
    let ret = unsafe {
        core::slice::from_raw_parts(ret as *const u8, core::mem::size_of::<WasiFdstat>())
    };
    mem.store(addr, core::mem::size_of::<WasiFdstat>(), ret)?;

    Ok(0)
}

pub fn fd_fdstat_set_flags(ctx: FuncContext<'_>, args: (i32, i32)) -> tinywasm::Result<i32> {
    println!("[Debug] Invoke into fd_fdstat_set_flags\n");

    let fd = args.0 as u32;
    let flag = args.1 as u16;

    Ok(0)
}

pub fn fd_prestat_get(mut ctx: FuncContext<'_>, args: (i32, i32)) -> tinywasm::Result<i32> {
    println!("[Debug] Invoke into fd_prestat_get\n");
    println!("args: fd: {:?}, ptr: {:?}", args.0, args.1);

    let mut mem = ctx.exported_memory_mut("memory")?;

    let prestat = WasiPrestatT {
        tag: 0, // 假设 tag 为 0，表示这是一个目录
        u: WasiPrestatUt {
            dir: WasiPrestatDir {
                dirname_len: 15, // 示例长度
            },
        },
    };

    let ret = (&prestat) as *const _ as usize;
    let ret = unsafe {
        core::slice::from_raw_parts(ret as *const u8, core::mem::size_of::<WasiPrestatT>())
    };
    // mem.store(args.1 as usize, core::mem::size_of::<WasiPrestatT>(), ret )?;

    // Badf
    Ok(8)
}

pub fn fd_prestat_dir_name(
    mut ctx: FuncContext<'_>,
    args: (i32, i32, i32),
) -> tinywasm::Result<i32> {
    // should not enter
    println!("[Debug] Invoke into fd_prestat_dir_name\n");
    println!(
        "args: fd: {:?}, path_addr: {:?}, path_len: {:?}",
        args.0, args.1, args.2
    );

    let mut mem = ctx.exported_memory_mut("memory")?;

    Ok(0)
}

pub fn fd_read(mut ctx: FuncContext<'_>, args: (i32, i32, i32, i32)) -> tinywasm::Result<i32> {
    println!("[Debug] Invoke into fd_read\n");

    let fd = args.0 as u32;
    let ptr = args.1 as usize;
    let iovs_len = args.2 as usize;
    let mut mem = ctx.exported_memory_mut("memory")?;
    let mut read_size: usize = 0;

    for i in 0..iovs_len {
        let offset = ptr + i * core::mem::size_of::<WasiCiovec>();
        let addr: &[u8] = mem.load(offset, core::mem::size_of::<WasiCiovec>())?;
        let addr: &WasiCiovec = unsafe { &*(addr.as_ptr() as *const WasiCiovec) };
        let buf: &[u8] = mem.load(addr.buf as usize, addr.buf_len as usize)?;
        let buf: &mut [u8] = unsafe {
            slice::from_raw_parts_mut(buf.as_ptr() as usize as *mut u8, addr.buf_len as usize)
        };
        read_size += libos!(read(fd, buf)).unwrap();
    }

    mem.store(
        args.3 as usize,
        core::mem::size_of::<u32>(),
        &read_size.to_ne_bytes(),
    )?;

    Ok(0)
}

pub fn fd_seek(mut ctx: FuncContext<'_>, args: (i32, i64, i32, i32)) -> tinywasm::Result<i32> {
    println!("[Debug] Invoke into fd_seek\n");

    let fd = args.0 as u32;
    let offset = args.1;
    let whence = args.2;
    let pos = offset as u32;
    // if whence == 0 {

    // } else if whence == 1 {

    // } else if whence == 2{

    // }

    libos!(lseek(fd, pos)).unwrap();

    Ok(0)
}

pub fn fd_write(mut ctx: FuncContext<'_>, args: (i32, i32, i32, i32)) -> tinywasm::Result<i32> {
    println!("[Debug] Invoke into fd_write\n");
    println!(
        "args: {:?}, {:?}, {:?}, {:?}",
        args.0, args.1, args.2, args.3
    );

    let fd = args.0 as u32;
    let ptr = args.1 as usize;
    let iovs_len = args.2 as usize;
    let mem = ctx.exported_memory("memory")?;

    for i in 0..iovs_len {
        let offset = ptr + i * core::mem::size_of::<WasiCiovec>();
        let iovs: &[u8] = mem.load(offset, core::mem::size_of::<WasiCiovec>())?;
        let iovs: &WasiCiovec = unsafe { &*(iovs.as_ptr() as *const WasiCiovec) };
        let buf = mem.load(iovs.buf as usize, iovs.buf_len as usize)? as &[u8];
        libos!(write(fd, buf)).unwrap();
    }

    Ok(0)
}

pub fn path_open(
    mut ctx: FuncContext<'_>,
    args: (i32, i32, i32, i32, i32, i64, i64, i32, i32),
) -> tinywasm::Result<i32> {
    println!("[Debug] Invoke into path_open\n");

    let mem = ctx.exported_memory("memory")?;
    let fd = args.0 as u32;
    let dirflags = args.1 as u32;
    let path_addr = args.2 as u32;
    let path_len = args.3 as u32;
    let path = mem.load_string(path_addr as usize, path_len as usize)?;
    let oflags = args.4 as u16;

    let fs_rights_base = args.5 as u64;
    let fs_rights_inheriting = args.6 as u64;
    let fdflags = args.7 as u16;

    let mut flags = OpenFlags::empty();
    if (fdflags & (1 << 0)) == 1 {
        flags |= OpenFlags::O_APPEND;
    }
    if (oflags & (1 << 0)) == 1 {
        flags |= OpenFlags::O_CREAT;
    }

    let mut mode = OpenMode::empty();
    #[allow(clippy::bad_bit_mask)]
    if (fs_rights_base & (1 << 1)) == 1 {
        mode |= OpenMode::RD;
    }
    #[allow(clippy::bad_bit_mask)]
    if (fs_rights_base & (1 << 6)) == 1 {
        mode |= OpenMode::WR;
    }

    let ret_fd = libos!(open(&path, flags, mode)).unwrap() as i32;
    Ok(ret_fd)
}

pub fn proc_exit(_: FuncContext<'_>, _args: i32) -> tinywasm::Result<()> {
    println!("[Debug] Invoke into proc_exit\n");

    panic!("normally exit");
}
