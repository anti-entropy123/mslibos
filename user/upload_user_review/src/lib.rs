#![cfg_attr(feature = "with_libos", no_std)]

cfg_if::cfg_if! {
    if #[cfg(feature = "with_libos")] {
        use ms_std::{agent::{FaaSFuncResult as Result, DataBuffer}, println};
        use ms_std_proc_macro::FaasData;

        extern crate alloc;
        use alloc::{ string::String, borrow::ToOwned};
    } else {
        type Result<T> = core::result::Result<T, String>;
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "with_libos")] {
        #[allow(dead_code)]
        #[derive(FaasData, Default, Clone, Debug)]
        struct MessageToStoreReview {
            review_id: String,
            user_id: usize,
        }
    } else {
        #[allow(dead_code)]
        #[derive(Debug)]
        struct MessageToStoreReview {
            review_id: &'static str,
            user_id: usize,
        }
        const MESSAGE: MessageToStoreReview = MessageToStoreReview{
            review_id: "3990522937178444",
            user_id: 112233
        };
    }
}

#[no_mangle]
pub fn main() -> Result<()> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "with_libos")] {
            let message =
                DataBuffer::<MessageToStoreReview>::from_buffer_slot("to_upload_review".to_owned())
                .ok_or("missing databuffer, slot: to_upload_review")?;

        } else {
            let message = MESSAGE;
        }
    }

    println!("upload_user_review: review message: {:?}", message);

    Ok(().into())
}
