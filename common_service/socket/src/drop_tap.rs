use std::{fs::File, os::fd::FromRawFd};

use log::info;

use crate::{get_tap_raw_fd, setup_tap::exec_tap_cleanup, NETDEV_NAME};

#[no_mangle]
pub fn drop() {
    info!("enter socket drop.");
    let netdev_name = NETDEV_NAME.try_lock().unwrap();
    // println!("netdev_name has init?: {}", netdev_name.is_some());
    if netdev_name.is_some() {
        // Sometimes exec_tap_cleanup will have error: "ioctl(TUNSETIFF): Device
        // or resource busy". By using below codes, could solve this error.
        {
            let fd = get_tap_raw_fd();
            let _file = unsafe { File::from_raw_fd(fd) };
        }
        // println!("has close tap fd.");
        let name = netdev_name.as_ref().unwrap();

        exec_tap_cleanup(name).expect("cleanup tap device failed.")
    }
}
