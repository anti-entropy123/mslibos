#![cfg_attr(feature = "with_libos", no_std)]

cfg_if::cfg_if! {
    if #[cfg(feature = "with_libos")] {
        use ms_std::{agent::{FaaSFuncResult as Result}};

        extern crate alloc;
        use alloc::{ string::String, borrow::ToOwned};
    } else {
        type Result<T> = core::result::Result<T, String>;
    }
}

use ms_std::agent::DataBuffer;
use ms_std_proc_macro::FaasData;

#[derive(FaasData, Default, Clone)]
struct UserId(usize);

#[derive(FaasData, Default, Clone)]
struct MovieTitleId {
    title_id: u64,
    rating: i32,
}

cfg_if::cfg_if! {
    if #[cfg(feature = "with_libos")] {
        #[derive(FaasData, Default, Clone)]
        struct UploadText {
            text: String,
        }
        #[derive(FaasData, Default, Clone)]
        struct UniqueReviewId(String);
    } else {
        struct UploadText {
            text: &'static str,
        }
        struct UniqueReviewId(&'static str);
        const USER_ID: UserId = UserId(112233);
        const MOVIE_ID: MovieTitleId = MovieTitleId { title_id: 16585301279346438498, rating: 7 };
        const UPLOAD_TEXT: UploadText = UploadText {text: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"};
        const UNIQUE_ID: UniqueReviewId = UniqueReviewId ("3990523756408444");
    }
}

#[derive(FaasData, Default, Clone)]
struct MessageToStoreReview {
    review_id: String,
    user_id: usize,
    movie_id: u64,
    text: String,
    rating: i32,
}

#[derive(FaasData, Default, Clone)]
struct MessageToUploadUserReview {
    review_id: String,
    user_id: usize,
}

#[derive(FaasData, Default, Clone)]
struct MessageToUploadMovieReview {
    review_id: String,
    movie_id: u64,
}

#[no_mangle]
pub fn main() -> Result<()> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "with_libos")] {
            let user_id =
                DataBuffer::<UserId>::from_buffer_slot("user_id".to_owned())
                .ok_or("missing databuffer, slot: user_id")?;

            let movie_id =
                DataBuffer::<MovieTitleId>::from_buffer_slot("movie_titie_id".to_owned())
                .ok_or("missing databuffer, slot: movie_titie_id")?;

            let upload_text =
                DataBuffer::<UploadText>::from_buffer_slot("mr_text".to_owned())
                .ok_or("missing databuffer, slot: mr_text")?;

            let unique_id =
                DataBuffer::<UniqueReviewId>::from_buffer_slot("unique_review_id".to_owned())
                .ok_or("missing databuffer, slot: unique_review_id")?;
        } else {
            let user_id = USER_ID;
            let movie_id = MOVIE_ID;
            let upload_text = UPLOAD_TEXT;
            let unique_id = UNIQUE_ID;
        }
    }

    let mut message1 = DataBuffer::<MessageToStoreReview>::with_slot("to_store_review".to_owned());
    cfg_if::cfg_if! {
        if #[cfg(feature = "with_libos")] {
            message1.review_id = unique_id.0.clone();
            message1.text = upload_text.text.clone();
        } else {
            message1.review_id = unique_id.0.to_string();
            message1.text = upload_text.text.to_string();
        }
    }
    message1.user_id = user_id.0;
    message1.movie_id = movie_id.title_id;
    message1.rating = movie_id.rating;

    let mut message2 =
        DataBuffer::<MessageToUploadUserReview>::with_slot("to_upload_review".to_owned());
    cfg_if::cfg_if! {
        if #[cfg(feature = "with_libos")] {
            message2.review_id = unique_id.0.clone();
        } else {
            message2.review_id = unique_id.0.to_string();
        }
    }
    message2.user_id = user_id.0;

    let mut message3 =
        DataBuffer::<MessageToUploadMovieReview>::with_slot("to_upload_movie_review".to_owned());
    cfg_if::cfg_if! {
        if #[cfg(feature = "with_libos")] {
            message3.review_id = unique_id.0.clone();
        } else {
            message3.review_id = unique_id.0.to_string();
        }
    }
    message3.movie_id = movie_id.title_id;

    Ok(().into())
}
