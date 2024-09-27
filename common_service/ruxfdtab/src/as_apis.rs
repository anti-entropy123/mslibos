extern crate alloc;

use std::path::PathBuf;

use alloc::vec;
use ms_std::libos::libos;
use spin::Mutex;

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

fn get_fs_image_path() -> PathBuf {
    let image_path = match libos!(fs_image(ms_std::init_context::isolation_ctx().isol_id)) {
        Some(s) => s,
        None => "fs_images/fatfs.img".to_owned(),
    };

    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
        .join(image_path)
}

fn init() {
    let all_devices = init_drivers(get_fs_image_path().to_str().unwrap());
    #[cfg(feature = "log")]
    println!("block devices nums: {}", all_devices.block.len());
    let mount_point = init_blkfs(all_devices.block);
    let mut mount_points = vec![mount_point];
    prepare_commonfs(&mut mount_points);
    init_filesystems(mount_points);
    #[cfg(feature = "log")]
    println!("ruxfs init ok");
}

lazy_static::lazy_static! {
    static ref MUST_EXIC: Mutex<bool> = {
        init();
        Mutex::new(true)
    };
}

#[no_mangle]
pub fn open(path: &str, flags: OpenFlags, mode: OpenMode) -> FdtabResult<Fd> {
    let _exec = MUST_EXIC.lock();
    #[cfg(feature = "log")]
    {
        println!("ruxfdtab: open path={:?}", path);
        for item in ruxfs::api::read_dir("/").unwrap().flatten() {
            println!("item: {}", item.file_name())
        }
    }

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

    let file = ruxfs::fops::File::open(path, &options)
        .map_err(|e| FdtabError::RuxfsError(format!("open {} failed: {}", path, e)))?;

    let result = fs::File::new(file).add_to_fd_table()?;
    #[cfg(feature = "log")]
    println!("ruxfdtab: open {} success, fd={:?}", path, result);

    Ok(result)
}

#[no_mangle]
pub fn write(fd: Fd, buf: &[u8]) -> FdtabResult<Size> {
    let _exec = MUST_EXIC.lock();
    // libos!(stdout(format!("fd: {}, buf: {:?}", fd, buf).as_bytes()));
    let f = get_file_like(fd)?;

    let result = f
        .write(buf)
        .map_err(|e| FdtabError::RuxfsError(e.to_string()));
    f.flush().unwrap();

    result
}

#[no_mangle]
pub fn read(fd: Fd, buf: &mut [u8]) -> FdtabResult<Size> {
    let _exec = MUST_EXIC.lock();
    get_file_like(fd)?
        .read(buf)
        .map_err(|e| FdtabError::RuxfsError(e.to_string()))
}

#[no_mangle]
pub fn close(fd: Fd) -> FdtabResult<()> {
    let _exec = MUST_EXIC.lock();
    close_file_like(fd)
}

#[no_mangle]
pub fn lseek(fd: Fd, pos: u32) -> FdtabResult<()> {
    let _exec = MUST_EXIC.lock();
    fs::File::from_fd(fd)?
        .inner
        .lock()
        .seek(axio::SeekFrom::Start(pos as u64))
        .map_err(|e| FdtabError::RuxfsError(e.to_string()))?;

    Ok(())
}

#[no_mangle]
pub fn stat(fd: Fd) -> FdtabResult<Stat> {
    let _exec = MUST_EXIC.lock();
    let stat = fs::File::from_fd(fd)?
        .stat()
        .map_err(|e| FdtabError::RuxfsError(e.to_string()))?;
    Ok(Stat {
        st_size: stat.st_size as usize,
    })
}
