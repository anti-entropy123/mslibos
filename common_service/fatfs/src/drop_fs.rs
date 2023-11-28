use crate::{get_fs_ref, FileSystem};

#[no_mangle]
pub fn drop() {
    let ref_addr = get_fs_ref() as *const _ as usize;
    let _ = unsafe { Box::from_raw(ref_addr as *mut FileSystem) };
}
