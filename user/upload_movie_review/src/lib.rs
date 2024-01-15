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
        struct MessageToUploadMovieReview {
            review_id: String,
            movie_id: u64,
        }
    } else {
        #[allow(dead_code)]
        #[derive(Debug)]
        struct MessageToUploadMovieReview {
            review_id: &'static str,
            movie_id: u64,
        }
        const MESSAGE: MessageToUploadMovieReview = MessageToUploadMovieReview{
            review_id: "3990522937178444",
            movie_id: 16585301279346438498
        };
    }
}

#[no_mangle]
pub fn main() -> Result<()> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "with_libos")] {
            let message =
                DataBuffer::<MessageToUploadMovieReview>::from_buffer_slot("to_upload_movie_review".to_owned())
                .ok_or("missing databuffer, slot: to_upload_movie_review")?;
        } else {
            let message = MESSAGE;
        }
    }

    println!("upload_movie_review: review message: {:?}", message);

    Ok(().into())
}
