#[cfg(feature = "log")]
use ms_std::println;
#[cfg(feature = "log")]
use alloc::format;

extern crate alloc;

use alloc::{string::{String, ToString}, vec::Vec};
use hashbrown::HashMap;
use spin::{Mutex, MutexGuard};
use wasmtime::Caller;

use ms_hostcall::{fdtab::FdtabResult, types::{OpenFlags, OpenMode, Fd}};
use ms_std::{
    libos::libos,
    time::{SystemTime, UNIX_EPOCH},
};
use crate::{types::*, LibosCtx};

pub(crate) struct WasiState {
    pub(crate) args: Vec<String>,
}

lazy_static::lazy_static! {
    static ref WASI_STATE: Mutex<HashMap<String, WasiState>> = Mutex::new( HashMap::new() );
}
fn get_hashmap_wasi_state_mut() -> MutexGuard<'static, HashMap<String, WasiState>> {
    WASI_STATE.lock()
}
fn get_wasi_state<'a>(
    id: &str,
    map: &'a MutexGuard<'static, HashMap<String, WasiState>>,
) -> &'a WasiState {
    let wasi_state = map.get(id).unwrap();
    if wasi_state.args.len() == 0 {
        panic!("WASI_STATE uninit")
    }
    wasi_state
}
pub fn set_wasi_state(id: &str, _args: Vec<String>) {
    let mut map = get_hashmap_wasi_state_mut();
    map.insert(id.to_string(), WasiState { args: _args });
}

pub fn args_get(mut caller: Caller<'_, LibosCtx>, argv: i32, argv_buf: i32) -> i32 {
    #[cfg(feature = "log")]
    {
        println!("[Debug] Invoke into args_get");
        println!("args: argv: {:?}, argv_buf: {:?}", argv, argv_buf);
    }

    // argv是每个arg在argv_buf中的起始地址的数组的起始地址
    // argv_buf是存arg的buf的起始地址
    // 在buf中存入arg，并以\0结尾 (len需要+1)

    let caller_id = &caller.data().id;
    let map = WASI_STATE.lock();
    let wasi_state = get_wasi_state(&caller_id, &map);
    let args: Vec<&[u8]> = wasi_state
        .args
        .iter()
        .map(|a| a.as_bytes())
        .collect::<Vec<_>>();
    let mut offset: usize = 0;

    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();

    #[cfg(feature = "log")]
    println!("arg_vec len: {}", args.len());
    for (i, arg) in args.iter().enumerate() {
        #[cfg(feature = "log")]
        println!(
            "i: {:?}, offset: {:?}, arg: {:?}, arg_len: {:?}",
            i,
            offset,
            arg,
            arg.len()
        );
        let arg_addr = argv_buf as usize + offset;

        memory.write(&mut caller, argv as usize + i * core::mem::size_of::<u32>(), &(arg_addr as u32).to_ne_bytes()).unwrap();
        memory.write(&mut caller, arg_addr, arg).unwrap();
        memory.write(&mut caller, arg_addr + arg.len(), "\0".as_bytes()).unwrap();

        offset += arg.len() + 1;
    }

    Errno::Success as i32
}

pub fn args_sizes_get(mut caller: Caller<'_, LibosCtx>, argc: i32, argv_buf_size: i32) -> i32 {
    #[cfg(feature = "log")]
    {
        println!("[Debug] Invoke into args_sizes_get");
        println!("args: argc: {:?}, argv_buf_size: {:?}", argc, argv_buf_size);
    }

    let caller_id = &caller.data().id;
    let map = WASI_STATE.lock();
    let wasi_state = get_wasi_state(&caller_id, &map);
    let argc_val = wasi_state.args.len();
    let argv_buf_size_val: usize = wasi_state.args.iter().map(|v| v.len() + 1).sum();

    #[cfg(feature = "log")]
    println!(
        "argc_val={:?}, argv_buf_size_val: {:?}",
        argc_val, argv_buf_size_val
    );

    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();

    memory.write(&mut caller, argc as usize, &(argc_val as u32).to_ne_bytes()).unwrap();
    memory.write(&mut caller, argv_buf_size as usize, &(argv_buf_size_val as u32).to_ne_bytes()).unwrap();

    Errno::Success as i32
}

pub fn fd_close(mut caller: Caller<'_, LibosCtx>, fd: i32) -> i32 {
    #[cfg(feature = "log")]
    {
        println!("[Debug] Invoke into fd_close");
        println!("args: fd: {:?}", fd);
    }

    libos!(close(fd as u32)).unwrap();
    Errno::Success as i32
}

pub fn fd_fdstat_get(mut caller: Caller<'_, LibosCtx>, fd: i32, retptr: i32) -> i32 {
    #[cfg(feature = "log")]
    {
        println!("[Debug] Invoke into fd_fdstat_get");
        println!("args: fd: {:?}, retptr: {:?}", fd, retptr);
    }
    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();

    let mut fdstat: WasiFdstat = WasiFdstat {
        fs_filetype: 0,
        fs_flags: 0,
        fs_rights_base: 0,
        fs_rights_inheriting: 0,
    };
    match fd as u32 {
        0 => {
            // stdin
            fdstat.fs_filetype = 2; // CharacterDevice
            fdstat.fs_flags = 0;
            fdstat.fs_rights_base = 0xFFFFFFFFFFFFFFFF;
            fdstat.fs_rights_inheriting = 0;
        }
        1 => {
            // stdout
            fdstat.fs_filetype = 2;
            fdstat.fs_flags = 1;
            fdstat.fs_rights_base = 0xFFFFFFFFFFFFFFFF;
            fdstat.fs_rights_inheriting = 0;
        }
        2 => {
            // stderr
            fdstat.fs_filetype = 2;
            fdstat.fs_flags = 1;
            fdstat.fs_rights_base = 0xFFFFFFFFFFFFFFFF;
            fdstat.fs_rights_inheriting = 0;
        }
        3 => {
            // root inode
            fdstat.fs_filetype = 3; // Directory
            fdstat.fs_flags = 0;
            fdstat.fs_rights_base = 0xFFFFFFFFFFFFFFFF;
            fdstat.fs_rights_inheriting = 0xFFFFFFFFFFFFFFFF;
        }
        _ => (),
    }
    // Todo: 从表中寻找fd
    // let msFdStat = table.find(fd);
    // fdstat.fs_filetype = match msFdStat.kind {
    //     0 => 4, // 0 代表File，4代表RegularFile
    //     1 => 3  // 1 代表Dir，3代表Directory
    // };
    // fdstat.fs_flags = msFdStat.flags;
    // fdstat.fs_rights_base = msFdStat.fs_rights_base;
    // fdstat.fs_rights_inheriting = msFdStat.fs_rights_inheriting;

    // if fd == 4 || fd == 5 || fd == 6 || fd == 7 || fd == 8 {
    // 假设打开的都是文件
    fdstat.fs_filetype = 4;
    fdstat.fs_flags = 0;
    fdstat.fs_rights_base = 0xFFFFFFFFFFFFFFFF;
    fdstat.fs_rights_inheriting = 0xFFFFFFFFFFFFFFFF;
    // }

    let buf = (&fdstat) as *const _ as usize;
    let buf = unsafe {
        core::slice::from_raw_parts(buf as *const u8, core::mem::size_of::<WasiFdstat>())
    };
    memory.write(&mut caller, retptr as usize, buf).unwrap();

    Errno::Success as i32
}

pub fn fd_fdstat_set_flags(mut caller: Caller<'_, LibosCtx>, fd: i32, offset: i32) -> i32 {
    #[cfg(feature = "log")]
    {
        println!("[Debug] Invoke into fd_fdstat_set_flags");
        println!("args: fd: {:?}, flag: {:?}", fd as u32, offset as u16);
    }

    Errno::Success as i32
}

pub fn fd_prestat_get(mut caller: Caller<'_, LibosCtx>, fd: i32, retptr: i32) -> i32 {
    #[cfg(feature = "log")]
    {
        println!("[Debug] Invoke into fd_prestat_get");
        println!("args: fd: {:?}, retptr: {:?}", fd, retptr);
    }
    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();

    match fd {
        3 => {
            // root inode
            let prestat = WasiPrestatT {
                tag: 0, // tag 应为 0，表示这是一个目录，非0表示unknown
                u: WasiPrestatUt {
                    dir: WasiPrestatDir {
                        dirname_len: "/".len() as u32,
                    },
                },
            };

            let buf = (&prestat) as *const _ as usize;
            let buf = unsafe {
                core::slice::from_raw_parts(buf as *const u8, core::mem::size_of::<WasiPrestatT>())
            };
            memory.write(&mut caller, retptr as usize, buf).unwrap();

            Errno::Success as i32
        }
        // Todo: libos需要维护一个表，从表中找fd，找不到就返回Badf
        _ => Errno::Badf as i32, // Badf
    }
}

pub fn fd_prestat_dir_name(
    mut caller: Caller<'_, LibosCtx>,
    fd: i32, path_addr: i32, path_len: i32
) -> i32 {
    #[cfg(feature = "log")]
    {
        println!("[Debug] Invoke into fd_prestat_dir_name");
        println!(
            "args: fd: {:?}, path_addr: {:?}, path_len: {:?}",
            fd, path_addr, path_len
        );
    }

    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();

    // Todo: 从表中寻找fd
    if fd == 3 {
        let name = "/";
        memory.write(&mut caller, path_addr as usize, name.as_bytes()).unwrap();

        Errno::Success as i32
    } else {
        Errno::Overflow as i32
    }
}

pub fn fd_read(mut caller: Caller<'_, LibosCtx>, fd: i32, iovs_ptr: i32, iovs_len: i32, retptr: i32) -> i32 {
    #[cfg(feature = "log")]
    {
        println!("[Debug] Invoke into fd_read");
        println!(
            "args: fd: {:?}, iovs_ptr: {:?}, iovs_len: {:?}, retptr: {:?}",
            fd, iovs_ptr, iovs_len, retptr
        );
    }

    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
    let mut read_size: usize = 0;

    for i in 0..iovs_len {
        let offset: usize = iovs_ptr as usize + i as usize * core::mem::size_of::<WasiCiovec>();
        let mut iovs = [0; core::mem::size_of::<WasiCiovec>()];
        memory.read(&caller, offset, &mut iovs).unwrap();
        let iovs: &WasiCiovec = unsafe { &*(iovs.as_ptr() as *const WasiCiovec) };
        let mut buf: Vec<u8> = Vec::with_capacity(iovs.buf_len as usize);
        buf.resize(iovs.buf_len as usize, 0);
        memory.read(&caller, iovs.buf as usize, &mut buf).unwrap();
        read_size += libos!(read(fd as u32, &mut buf)).unwrap();
    }

    #[cfg(feature = "log")]
    println!("read_size: {:?}", read_size);
    memory.write(&mut caller, retptr as usize, &read_size.to_ne_bytes()).unwrap();
    Errno::Success as i32
}

pub fn fd_seek(mut caller: Caller<'_, LibosCtx>, fd: i32, offset: i64, whence: i32, pos: i32) -> i32 {
    #[cfg(feature = "log")]
    {
        println!("[Debug] Invoke into fd_seek");
        println!(
            "args: fd: {:?}, offset: {:?}, whence: {:?}, pos: {:?}",
            fd, offset, whence, pos
        );
    }

    // TO BE FIX FOR PY HELLO
    // let fd = args.0 as u32;
    // let offset = args.1;
    // let whence = args.2;
    // let pos = offset as u32;
    // // if whence == 0 {

    // // } else if whence == 1 {

    // // } else if whence == 2{

    // // }

    // libos!(lseek(fd, pos)).unwrap();

    Errno::Success as i32
}

pub fn fd_write(mut caller: Caller<'_, LibosCtx>, fd: i32, iovs_ptr: i32, iovs_len: i32, retptr: i32) -> i32 {
    #[cfg(feature = "log")]
    {
        println!("[Debug] Invoke into fd_write");
        println!(
            "args: fd: {:?}, iovs_ptr: {:?}, iovs_len: {:?}, retptr: {:?}",
            fd, iovs_ptr, iovs_len, retptr
        );
    }
    
    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
    let mut write_size: usize = 0;

    for i in 0..iovs_len {
        let offset: usize = iovs_ptr as usize + i as usize * core::mem::size_of::<WasiCiovec>();
        let mut iovs = [0; core::mem::size_of::<WasiCiovec>()];
        memory.read(&caller, offset, &mut iovs).unwrap();
        let iovs: &WasiCiovec = unsafe { &*(iovs.as_ptr() as *const WasiCiovec) };
        let mut buf: Vec<u8> = Vec::with_capacity(iovs.buf_len as usize);
        buf.resize(iovs.buf_len as usize, 0);
        memory.read(&caller, iovs.buf as usize, &mut buf).unwrap();
        write_size += libos!(write(fd as u32, &buf)).unwrap();
    }

    #[cfg(feature = "log")]
    println!("write_size: {:?}", write_size);
    memory.write(&mut caller, retptr as usize, &write_size.to_ne_bytes()).unwrap();
    Errno::Success as i32
}

pub fn path_open(
    mut caller: Caller<'_, LibosCtx>,
    fd: i32, dirflags: i32, path_addr: i32, path_len: i32, oflags: i32, fs_rights_base: i64, fs_rights_inheriting: i64, fdflags: i32, retptr: i32
) -> i32 {
    #[cfg(feature = "log")]
    {
        println!("[Debug] Invoke into path_open");
        println!("args: fd: {:?}, dirflags: {:?}, path_addr: {:?}, path_len: {:?}, oflags: {:?}, fs_rights_base: {:?}, fs_rights_inheriting: {:?}, fdflags: {:?}, retptr: {:?}", fd as u32, dirflags as u32, path_addr as u32, path_len as u32, oflags as u16, format!("{:064b}", fs_rights_base as u64), format!("{:064b}", fs_rights_inheriting as u64), fdflags as u16, retptr as u32);
    }
    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();

    let mut path: Vec<u8> = Vec::with_capacity(path_len as usize);
    path.resize(path_len as usize, 0);
    memory.read(&caller, path_addr as usize, &mut path).unwrap();
    let path = String::from_utf8(path).expect("[Err] Not a valid UTF-8 sequence");
    #[cfg(feature = "log")]
    println!("path: {:?}", path);

    let mut flags: OpenFlags = OpenFlags::empty();
    if (fdflags as u16 & 1) == 1 {
        flags |= OpenFlags::O_APPEND;
    }
    if (oflags as u16 & 1) == 1 {
        flags |= OpenFlags::O_CREAT;
    }

    let mut mode: OpenMode = OpenMode::empty();
    if ((fs_rights_base as u64 >> 1) & 1) == 1 {
        mode |= OpenMode::RD;
    }
    if ((fs_rights_base as u64 >> 6) & 1) == 1 {
        mode |= OpenMode::WR;
    }

    // Todo: 将对应信息添加到table中，结构体struct {name, fs_flags, fs_rights_base, fs_rights_inheriting, kind}
    // 其中，name是路径，fs_flags是Fdflags类型，fs_rights_base和fs_rights_inheriting直接存输入参数。
    // kind表示文件类型，是个enum。eg. File Dir
    let path_fd: FdtabResult<Fd> = libos!(open(&path, flags, mode));
    let path_fd = if let Err(_e) = path_fd {
        #[cfg(feature = "log")] {
            println!("get path err: {:?}", _e);
            println!("return to wasi with errno 44: (noent - No such file or directory)");
        }
        core::mem::forget(_e);
        return Errno::Noent as i32;
    } else {
        path_fd.unwrap()
    };

    #[cfg(feature = "log")]
    println!("return path_fd: {:?}", path_fd);
    memory.write(&mut caller, retptr as usize, &path_fd.to_ne_bytes()).unwrap();
    Errno::Success as i32
}

pub fn proc_exit(mut caller: Caller<'_, LibosCtx>, code: i32) {
    #[cfg(feature = "log")]
    {
        println!("[Debug] Invoke into proc_exit");
        // An exit code of 0 indicates successful termination of the program.
        println!("args: code: {:?}", code);
    }
    // nothing to do
}

