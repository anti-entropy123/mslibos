use std::time::{Duration, SystemTime};

use crate::fs_ops::{build_fatfs, fatfs_read, fatfs_write, native_read, native_write};

mod ns_ops;
mod fs_ops;

fn test_fatfs_read() {
    let fs = build_fatfs();
    let cost_time = measure_cost_time(|| fatfs_read(fs));
    println!("cost time: {:?}", cost_time)
}

fn test_native_read() {
    let cost_time = measure_cost_time(native_read);
    println!("cost time: {:?}", cost_time)
}

fn test_fatfs_write() {
    let fs = build_fatfs();
    let buf = build_buf();

    let cost_time = measure_cost_time(|| fatfs_write(fs, &buf));
    println!("cost time: {:?}", cost_time)
}

fn test_native_write() {
    let buf = build_buf();
    let cost_time = measure_cost_time(|| native_write(&buf));
    println!("cost time: {:?}", cost_time)
}

fn main() {
    test_fatfs_read();
    test_native_read();
    test_fatfs_write();
    test_native_write();
}

fn measure_cost_time<F>(f: F) -> Duration
where
    F: FnOnce(),
{
    let start = SystemTime::now();
    f();
    SystemTime::now().duration_since(start).unwrap()
}

fn build_buf() -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    for idx in 0..50 * 1024 * 1024 {
        buf.push((idx % 126usize) as u8)
    }
    buf
}
