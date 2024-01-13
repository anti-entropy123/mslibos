#![cfg_attr(feature = "with_libos", no_std)]

cfg_if::cfg_if! {
    if #[cfg(feature = "with_libos")] {
        use ms_std::{agent::FaaSFuncResult as Result};
        use ms_std::agent::DataBuffer;
        use ms_std_proc_macro::FaasData;
        extern crate alloc;
        use alloc::{string::String, borrow::ToOwned};
    } else {
        type Result<T> = core::result::Result<T, String>;
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "with_libos")] {
        #[derive(FaasData, Default, Clone)]
        struct MessageToMrUploadText {
            text: String,
        }
    } else {
        struct MessageToMrUploadText{
            text: &'static str,
        }
        const fn mock_message() -> MessageToMrUploadText {
            MessageToMrUploadText {
                text: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
            }
        }
        const MESSAGE: MessageToMrUploadText = mock_message();
    }
}

#[no_mangle]
pub fn main() -> Result<()> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "with_libos")] {
            let _text =
                DataBuffer::<MessageToMrUploadText>::from_buffer_slot("upload_text".to_owned())
                .ok_or("missing databuffer, slot: upload_text")?;
        } else {
            let _text = MESSAGE;
        }
    }

    Ok(().into())
}
