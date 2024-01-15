#![cfg_attr(feature = "with_libos", no_std)]

cfg_if::cfg_if! {
    if #[cfg(feature = "with_libos")] {
        use ms_std::{agent::FaaSFuncResult as Result};

        extern crate alloc;
        use alloc::{string::String, borrow::ToOwned};
    } else {
        type Result<T> = core::result::Result<T, String>;
    }
}

use ms_std::agent::DataBuffer;
use ms_std_proc_macro::FaasData;

#[derive(FaasData, Default, Clone)]
struct MessageToMrUploadText {
    text: String,
}

impl MessageToMrUploadText {
    fn to_string(&self) -> String {
        self.text.clone()
    }
}

cfg_if::cfg_if! {
    if #[cfg(not(feature = "with_libos"))] {
        struct MockMessageToMrUploadText{
            text: &'static str,
        }

        impl MockMessageToMrUploadText {
            fn to_string(&self) -> String {
                self.text.to_string()
            }
        }

        const fn mock_message() -> MockMessageToMrUploadText {
            MockMessageToMrUploadText {
                text: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
            }
        }
        const MESSAGE: MockMessageToMrUploadText = mock_message();
    }
}

#[no_mangle]
pub fn main() -> Result<()> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "with_libos")] {
            let text =
                DataBuffer::<MessageToMrUploadText>::from_buffer_slot("upload_text".to_owned())
                .ok_or("missing databuffer, slot: upload_text")?;
        } else {
            let text = MESSAGE;
        }
    }

    let mut message = DataBuffer::<MessageToMrUploadText>::with_slot("mr_text".to_owned());
    message.text = text.to_string();

    Ok(().into())
}
