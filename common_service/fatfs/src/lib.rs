#![feature(seek_stream_len)]
#![allow(clippy::result_unit_err)]

use std::{fs, mem::ManuallyDrop, path::PathBuf, sync::Mutex};

use fscommon::BufStream;
use lazy_static::lazy_static;

use ms_hostcall::types::Fd;
use ms_std::libos::libos;

type FileSystem = fatfs::FileSystem<fscommon::BufStream<std::fs::File>>;
type File<'a> = fatfs::File<'a, fscommon::BufStream<std::fs::File>>;
type FatfsHandle = usize;

pub mod apis;
pub mod drop_fs;

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
        // valid because `ManuallyDrop` can guarantee 'static lifetime.
        let mut file_system: ManuallyDrop<Box<_>> = ManuallyDrop::new(Box::new(
        {
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
        }));

        file_system.as_mut() as *mut _ as usize
    };

    static ref FTABLE: Mutex<FatfsFileList> = Default::default();
}

fn get_fs_ref() -> &'static FileSystem {
    unsafe { &*(*FS_REF_ADDR as *const FileSystem) }
}
