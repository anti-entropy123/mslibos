extern crate alloc;

use alloc::{string::String, vec::Vec};

use ms_hostcall::types::{OpenFlags, OpenMode};
use ms_std::{
    libos::libos,
    println,
    time::{SystemTime, UNIX_EPOCH},
};
use wasmtime::Caller;
use crate::errno::Errno;

#[repr(C)]
struct WasiCiovec {
    buf: u32,
    buf_len: u32,
}

pub fn fd_write(mut caller: Caller<'_, ()>, fd: i32, iovs_ptr: i32, iovs_len: i32, retptr: i32) -> i32 {
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
        let mut iovs = [0; 8];
        memory.read(&caller, offset, &mut iovs).unwrap();
        let iovs: &WasiCiovec = unsafe { &*(iovs.as_ptr() as *const WasiCiovec) };
        let mut buf: Vec<u8> = Vec::with_capacity(iovs.buf_len as usize);
        buf.resize(iovs.buf_len as usize, 0);
        memory.read(&caller, iovs.buf as usize, &mut buf).unwrap();
        write_size += libos!(write(fd as u32, buf.as_slice())).unwrap();
    }

    #[cfg(feature = "log")]
    println!("write_size: {:?}", write_size);
    memory.write(&mut caller, retptr as usize, &write_size.to_ne_bytes()).unwrap();
    Errno::Success as i32
}

pub fn proc_exit(mut caller: Caller<'_, ()>, code: i32) {
    #[cfg(feature = "log")]
    {
        println!("[Debug] Invoke into proc_exit");
        // An exit code of 0 indicates successful termination of the program.
        println!("args: code: {:?}", code);
    }
    // panic!("normally exit")
    // Todo
}

