use std::{
    fs::File,
    io::{Read, Write},
};

use fscommon::BufStream;

pub fn fatfs_read(fs: FatFs) {
    let root_dir = fs.root_dir();
    let mut file = root_dir.open_file("fake_data_0.txt").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    println!("fatfs_read size = {}", buffer.len())
}

pub fn native_read() {
    let mut file = File::open("fake_data_0.txt").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    println!("native_read size = {}", buffer.len())
}

pub fn fatfs_write(fs: FatFs, buf: &[u8]) {
    let root_dir = fs.root_dir();
    let mut file = root_dir.create_file("test_write_perf.txt").unwrap();
    file.write_all(buf).unwrap();
    file.flush().unwrap();
    println!("fatfs_write size = {}", buf.len())
}

pub fn native_write(buf: &[u8]) {
    let mut file = File::create("test_write_perf.txt").unwrap();
    file.write_all(buf).unwrap();
    file.flush().unwrap();
    println!("native_write size = {}", buf.len())
}

const IMAGE_PATH: &str = "../../fs_images/fatfs_large.img";

type FatFs = fatfs::FileSystem<fscommon::BufStream<std::fs::File>>;
pub fn build_fatfs() -> FatFs {
    let image = {
        let mut config = File::options();
        BufStream::new(
            config
                .read(true)
                .write(true)
                .open(IMAGE_PATH)
                .unwrap_or_else(|e| panic!("open img {:?} failed, err: {}", IMAGE_PATH, e)),
        )
    };
    fatfs::FileSystem::new(image, fatfs::FsOptions::new()).unwrap()
}

// type FatFs = fatfs::FileSystem<std::fs::File>;
// pub fn build_fatfs() -> FatFs {
//     let image = File::options()
//         .write(true)
//         .read(true)
//         .open(IMAGE_PATH)
//         .unwrap();
//     fatfs::FileSystem::new(image, fatfs::FsOptions::new()).unwrap()
// }
