#![cfg_attr(feature = "with_libos", no_std)]

cfg_if::cfg_if! {
    if #[cfg(feature = "with_libos")] {
        use ms_std::{agent::{FaaSFuncResult as Result, DataBuffer}, println};
        use ms_std_proc_macro::FaasData;
        extern crate alloc;
        use alloc::{collections::BTreeMap, string::String, borrow::ToOwned};
    } else {
        type Result<T> = core::result::Result<T, String>;
        use std::collections::BTreeMap;
    }
}

#[derive(FaasData, Default, Clone)]
struct MessageToUploadUserId(String);

#[no_mangle]
pub fn main() -> Result<()> {
    let user_name =
        DataBuffer::<MessageToUploadUserId>::from_buffer_slot("upload_user_name".to_owned())
            .ok_or("missing databuffer, slot: upload_user_name")?;

    // should query database.
    let user_id = if user_name.0.eq("abcd") {
        "112233"
    } else {
        Err("unknown username.")?
    };

    Ok(().into())
}
