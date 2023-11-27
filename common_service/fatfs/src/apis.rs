use std::{
    io::{Read, Seek, Write},
    mem::ManuallyDrop,
};

use ms_hostcall::types::{Fd, OpenFlags, Size, Stat};

use crate::{get_fs_ref, FTABLE};

#[no_mangle]
pub fn fatfs_read(fd: Fd, buf: &mut [u8]) -> Result<Size, ()> {
    let mut table = FTABLE.lock().expect("require lock failed.");
    let file = table.get_file_mut(fd);

    let mut read_size = 0;
    let mut buf = buf;
    while !buf.is_empty() {
        match file.read(buf) {
            Ok(0) => break,
            Ok(size) => {
                read_size += size;
                buf = &mut buf[size..]
            }
            Err(e) => panic!("fatfs read failed: {}", e),
        }
    }

    Ok(read_size)
}

#[no_mangle]
pub fn fatfs_seek(fd: Fd, pos: u32) -> Result<(), ()> {
    // println!("fatfs: try seek to {}", pos);
    let mut table = FTABLE.lock().expect("require lock failed.");
    let f = table.get_file_mut(fd);
    f.seek(std::io::SeekFrom::Start(pos as u64))
        .expect("seek failed");

    Ok(())
}

#[no_mangle]
pub fn fatfs_stat(fd: Fd) -> Result<Stat, ()> {
    let mut table = FTABLE.lock().expect("require lock failed.");
    let f = table.get_file_mut(fd);
    let st_size = f.stream_len().expect("fatfs: get stream len failed.") as Size;
    Ok(Stat { st_size })
}

#[no_mangle]
pub fn fatfs_open(p: &str, flags: OpenFlags) -> Result<Fd, ()> {
    let mut table = FTABLE.lock().expect("require lock failed.");
    let root_dir = get_fs_ref().root_dir();

    let file = if flags.contains(OpenFlags::O_CREAT) {
        root_dir.create_file(p).expect("create file failed.")
    } else {
        root_dir.open_file(p).expect("open file failed.")
    };

    let fd = {
        let file = ManuallyDrop::new(Box::new(file));
        table.add_file(file.as_ref())
    };

    Ok(fd as u32)
}

#[test]
fn fatfs_open_test() {
    let mut table = FTABLE.lock().expect("require lock failed.");
    let fd = fatfs_open("new_file.txt", OpenFlags::O_CREAT).expect("open file failed") as usize;
    let f = table.get_file_mut(fd as u32);

    let mut buf = String::new();
    f.read_to_string(&mut buf).expect("read failed");
    // assert!(!buf.is_empty());
}

#[no_mangle]
pub fn fatfs_write(fd: Fd, buf: &[u8]) -> Result<Size, ()> {
    let mut table = FTABLE.lock().expect("require lock failed.");

    let file = table.get_file_mut(fd);
    file.write_all(buf).expect("write file failed");
    file.flush().expect("flush failed");

    Ok(buf.len())
}

#[test]
fn fatfs_write_test() {
    let root_dir = get_fs_ref().root_dir();
    let mut file = root_dir
        .create_file("hello.txt")
        .expect("create file failed");

    assert!(file.write(b"Hello World!").expect("write failed") > 0);
}

#[no_mangle]
pub fn fatfs_close(fd: Fd) -> Result<(), ()> {
    let mut table = FTABLE.lock().expect("require lock failed.");

    table.remove_file(fd as usize)
}
