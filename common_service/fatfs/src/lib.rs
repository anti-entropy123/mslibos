#![feature(seek_stream_len)]
#![allow(clippy::result_unit_err)]

use std::{
    fs,
    io::{Read, Seek, Write},
    mem::ManuallyDrop,
    path::PathBuf,
    sync::Mutex,
};

use fscommon::BufStream;
use lazy_static::lazy_static;

use ms_hostcall::types::{Fd, OpenFlags, Size, Stat};
use ms_std::libos::libos;

type FileSystem = fatfs::FileSystem<fscommon::BufStream<std::fs::File>>;
type File<'a> = fatfs::File<'a, fscommon::BufStream<std::fs::File>>;
type FatfsHandle = usize;

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

thread_local! {
    static FS_RAW: FileSystem = {
        let image = {
            let mut config = fs::File::options();
            let image_path = get_fs_image_path();
            BufStream::new(config
                .read(true)
                .write(true)
                .open(image_path.clone())
                .unwrap_or_else(|e| panic!("open img {:?} failed, err: {}", image_path, e)))
        };
        FileSystem::new(image, fatfs::FsOptions::new()).expect("fatfs::new() failed.")
    };
}

#[derive(Default)]
struct FatfsFileList {
    table: Vec<Option<usize>>,
}

impl FatfsFileList {
    fn get_file_raw_ptr(&self, fd: Fd) -> usize {
        if let Some(Some(file_addr)) = self.table.get(fd as usize) {
            // println!("get_file_mut: file addr=0x{:x}", file_addr);
            *file_addr
        } else {
            panic!("fatfs_fd={} don't exist", fd);
        }
    }

    fn _get_file(&self, fd: Fd) -> &'static File<'static> {
        unsafe { &*(self.get_file_raw_ptr(fd) as *const File) }
    }

    fn get_file_mut(&mut self, fd: Fd) -> &'static mut File<'static> {
        unsafe { &mut *(self.get_file_raw_ptr(fd) as *mut File) }
    }

    fn add_file(&mut self, file: &File) -> FatfsHandle {
        self.table.push(Some(file as *const _ as usize));
        self.table.len() - 1
    }

    fn remove_file(&mut self, handle: FatfsHandle) -> Result<(), ()> {
        let file = self.table.get_mut(handle);
        if let Some(file) = file {
            match file.take() {
                Some(file_addr) => {
                    let _ = unsafe { Box::from_raw(file_addr as *mut File) };
                    Ok(())
                }
                None => Err(()),
            }
        } else {
            Err(())
        }
    }
}

lazy_static! {
    static ref FS_REF_ADDR: usize = {
        // I think this hack for getting reference to file system instance is
        // valid because thread local store can guarantee 'static lifetime.
        FS_RAW.with(|fs| fs as *const _ as usize)
    };

    static ref FTABLE: Mutex<FatfsFileList> = Default::default();
}

fn get_fs_ref() -> &'static FileSystem {
    unsafe { &*(*FS_REF_ADDR as *const FileSystem) }
}

#[no_mangle]
pub fn fatfs_read(fd: Fd, buf: &mut [u8]) -> Result<Size, ()> {
    let mut table = FTABLE.lock().expect("require lock failed.");
    let file = table.get_file_mut(fd);

    Ok(file.read(buf).expect("fatfs_read failed."))
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
