use crate::{FileSystem, FS_REF_ADDR};

#[no_mangle]
pub fn drop() {
    let _ = unsafe { Box::from_raw(*FS_REF_ADDR as *mut FileSystem) };
}
