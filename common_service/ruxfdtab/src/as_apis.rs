extern crate alloc;

use alloc::vec;

use crate::{
    fd_ops::{close_file_like, get_file_like},
    fs,
};
use ms_hostcall::{
    fdtab::{FdtabError, FdtabResult},
    types::{Fd, OpenFlags, OpenMode, Size, Stat},
};
use ruxdriver::init_drivers;
use ruxfdtable::FileLike;
use ruxfs::{fops::OpenOptions, init_blkfs, init_filesystems, prepare_commonfs};

fn init() {
    let all_devices = init_drivers();
    // println!("block devices nums: {}", all_devices.block.len());
    let mount_point = init_blkfs(all_devices.block);
    let mut mount_points = vec![mount_point];
    prepare_commonfs(&mut mount_points);
    init_filesystems(mount_points);
}

lazy_static::lazy_static! {
    static ref MUST_EXIC: bool = {
        init();
        true
    };
}

#[no_mangle]
pub fn open(path: &str, flags: OpenFlags, mode: OpenMode) -> FdtabResult<Fd> {
    let _exec = *MUST_EXIC;
    let mut options = OpenOptions::new();
    if mode.contains(OpenMode::WR) {
        options.write(true)
    }
    if mode.contains(OpenMode::RD) {
        options.read(true)
    }
    if flags.contains(OpenFlags::O_CREAT) {
        options.create(true);
    }

    let file = ruxfs::fops::File::open(path, &options).map_err(|_| FdtabError::Unknown)?;
    // println!("open successful, result={:?}", result);
    // for item in ruxfs::api::read_dir("/").unwrap().flatten() {
    //     println!("item: {}", item.file_name())
    // }
    fs::File::new(file).add_to_fd_table()
}

#[no_mangle]
pub fn write(fd: Fd, buf: &[u8]) -> FdtabResult<Size> {
    // libos!(stdout(format!("fd: {}, buf: {:?}", fd, buf).as_bytes()));
    let f = get_file_like(fd).map_err(|_| FdtabError::Unknown)?;

    let result = f.write(buf).map_err(|_| FdtabError::Unknown);
    f.flush().unwrap();

    result
}

#[no_mangle]
pub fn read(fd: Fd, buf: &mut [u8]) -> FdtabResult<Size> {
    get_file_like(fd)
        .map_err(|_| FdtabError::Unknown)?
        .read(buf)
        .map_err(|_| FdtabError::Unknown)
}

#[no_mangle]
pub fn close(fd: Fd) -> FdtabResult<()> {
    close_file_like(fd)
}

#[no_mangle]
pub fn lseek(fd: Fd, pos: u32) -> FdtabResult<()> {
    fs::File::from_fd(fd)
        .map_err(|_| FdtabError::NoExistFd(fd))?
        .inner
        .lock()
        .seek(axio::SeekFrom::Start(pos as u64))
        .map_err(|_| FdtabError::Unknown)?;

    Ok(())
}

#[no_mangle]
pub fn stat(fd: Fd) -> FdtabResult<Stat> {
    let stat = fs::File::from_fd(fd)
        .map_err(|_| FdtabError::NoExistFd(fd))?
        .stat()
        .map_err(|_| FdtabError::Unknown)?;
    Ok(Stat {
        st_size: stat.st_size as usize,
    })
}
