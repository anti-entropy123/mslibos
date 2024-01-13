#![cfg_attr(feature = "with_libos", no_std)]

cfg_if::cfg_if! {
    if #[cfg(feature = "with_libos")] {
        use ms_std::{agent::{FaaSFuncResult as Result}};
        use ms_std::agent::DataBuffer;
        use ms_std_proc_macro::FaasData;

        extern crate alloc;
        use alloc::{ string::String, borrow::ToOwned};
    } else {
        type Result<T> = core::result::Result<T, String>;
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "with_libos")] {
        #[derive(FaasData, Default, Clone)]
        struct MessageToUploadUserId(String);
    } else {
        struct MessageToUploadUserId(&'static str);
        const fn mock_message() -> MessageToUploadUserId {
            MessageToUploadUserId ("abcd")
        }
        const MESSAGE: MessageToUploadUserId = mock_message();
    }
}

#[no_mangle]
pub fn main() -> Result<()> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "with_libos")] {
            let user_name =
                DataBuffer::<MessageToUploadUserId>::from_buffer_slot("upload_user_name".to_owned())
                .ok_or("missing databuffer, slot: upload_user_name")?;
        } else {
            let user_name = MESSAGE;
        }
    }

    // should query database.
    let _user_id = if user_name.0.eq("abcd") {
        "112233"
    } else {
        Err("unknown username.")?
    };

    Ok(().into())
}
