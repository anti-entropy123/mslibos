#![allow(clippy::result_unit_err)]

use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
    sync::Mutex,
};

use fscommon::BufStream;
use ms_hostcall::types::{Fd, OpenFlags, Size};
pub use ms_std;

type FileSystem = fatfs::FileSystem<fscommon::BufStream<std::fs::File>>;
type File<'a> = fatfs::File<'a, fscommon::BufStream<std::fs::File>>;

thread_local! {
    static FS_RAW: FileSystem = {
        let image = {
            let mut config = fs::File::options();
            let image_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .parent().unwrap()
                .parent().unwrap()
                .to_path_buf()
                .join("fs_images/fatfs.img");

            BufStream::new(config
                .read(true)
                .write(true)
                .open(image_path.clone())
                .unwrap_or_else(|_| panic!("open img {:?} failed", image_path)))
        };
        FileSystem::new(image, fatfs::FsOptions::new()).expect("fatfs::new() failed.")
    };

    static FTABLE: Mutex<Vec<Option<File<'static>>>> = Mutex::new(Vec::default());
}

fn get_fs_ref() -> &'static FileSystem {
    // I think this hack for getting reference to file system instance is
    // valid because thread local store can guarantee 'static lifetime.
    let fs_addr = FS_RAW.with(|fs| fs as *const _ as usize);
    unsafe { &*(fs_addr as *const FileSystem) }
}

#[no_mangle]
pub fn fatfs_open(p: &str, flags: OpenFlags) -> Result<Fd, ()> {
    let root_dir = get_fs_ref().root_dir();

    let file = if flags.contains(OpenFlags::O_CREAT) {
        root_dir.create_file(p).expect("create file failed.")
    } else {
        root_dir.open_file(p).expect("open file failed.")
    };

    let fd = FTABLE.with(|table| {
        let mut table = table.lock().expect("require lock failed.");
        table.push(Some(file));
        table.len() - 1
    });

    Ok(fd as u32)
}

#[test]
fn fatfs_open_test() {
    let fd = fatfs_open("new_file.txt", OpenFlags::O_CREAT).expect("open file failed") as usize;
    FTABLE.with(|t| {
        let mut t = t.lock().expect("require lock failed");
        assert!(t.len() == fd + 1);
        if let Some(Some(ref mut f)) = t.get_mut(fd) {
            let mut buf = String::new();
            f.read_to_string(&mut buf).expect("read failed");
            // assert!(!buf.is_empty());
        };
    })
}

fn write(mut file: File, data: &[u8]) -> Size {
    file.write_all(data).expect("write file failed");
    data.len()
}

#[no_mangle]
pub fn fatfs_write(fd: Fd, data: &[u8]) -> Result<Size, ()> {
    let file = FTABLE.with(|ft| {
        let ft = ft.lock().expect("require lock failed.");
        if let Some(Some(file)) = ft.get(fd as usize) {
            file.clone()
        } else {
            panic!("fd don't exist");
        }
    });

    Ok(write(file, data))
}

#[test]
fn fatfs_write_test() {
    let root_dir = get_fs_ref().root_dir();
    let file = root_dir
        .create_file("hello.txt")
        .expect("create file failed");

    assert!(write(file, b"Hello World!") > 0);
}
