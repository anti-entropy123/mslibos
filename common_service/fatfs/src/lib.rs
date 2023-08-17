#![allow(clippy::result_unit_err)]

use std::{
    fs,
    io::{Read, Write},
    sync::Mutex,
};

use fscommon::BufStream;

type FileSystem = fatfs::FileSystem<fscommon::BufStream<std::fs::File>>;
type File<'a> = fatfs::File<'a, fscommon::BufStream<std::fs::File>>;

thread_local! {
    static FS_RAW: FileSystem = {
        let image = {
            let mut config = fs::File::options();

            BufStream::new(config
                .read(true)
                .write(true)
                .open("fatfs.img")
                .expect("open img failed"))
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
pub fn fatfs_open() -> Result<u32, ()> {
    let root_dir = get_fs_ref().root_dir();
    let file = root_dir.open_file("hello.txt").expect("open file failed.");

    let fd = FTABLE.with(|table| {
        let mut table = table.lock().expect("require lock failed.");
        table.push(Some(file));
        table.len() - 1
    });

    Ok(fd as u32)
}

#[test]
fn fatfs_open_test() {
    let fd = fatfs_open().expect("open file failed") as usize;
    FTABLE.with(|t| {
        let mut t = t.lock().expect("require lock failed");
        assert!(t.len() == fd + 1);
        if let Some(Some(ref mut f)) = t.get_mut(fd) {
            let mut buf = String::new();
            f.read_to_string(&mut buf).expect("read failed");
            assert!(!buf.is_empty());
        };
    })
}

#[no_mangle]
pub fn fatfs_write() {
    let root_dir = get_fs_ref().root_dir();
    let mut file = root_dir
        .create_file("hello.txt")
        .expect("create file failed");

    file.write_all(b"Hello World!").expect("write file failed");
}

#[test]
fn fatfs_write_test() {
    fatfs_write()
}
