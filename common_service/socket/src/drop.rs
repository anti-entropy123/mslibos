use crate::{setup_tap::exec_tap_cleanup, NETDEV_NAME};

#[no_mangle]
pub fn drop() {
    let netdev_name = NETDEV_NAME.lock().unwrap();
    // println!("netdev_name has init?: {}", netdev_name.is_some());
    if netdev_name.is_some() {
        let name = netdev_name.as_ref().unwrap();
        exec_tap_cleanup(name).expect("cleanup tap device failed.")
    }
}
