#![cfg_attr(feature = "with_libos", no_std)]

use core::hash::{BuildHasher, Hash, Hasher};

cfg_if::cfg_if! {
    if #[cfg(feature = "with_libos")] {
        use ms_std::{agent::{FaaSFuncResult as Result}};

        extern crate alloc;
        use alloc::{string::String, borrow::ToOwned};
    } else {
        type Result<T> = core::result::Result<T, String>;
    }
}

use ms_std::agent::DataBuffer;
use ms_std_proc_macro::FaasData;

cfg_if::cfg_if! {
    if #[cfg(feature = "with_libos")] {
        #[derive(FaasData, Default, Clone)]
        struct MessageToUploadMovieId {
            title: String,
            rating: i32,
        }
    } else {
        struct MessageToUploadMovieId {
            title: &'static str,
            rating: i32,
        }
        const fn mock_message() -> MessageToUploadMovieId {
            MessageToUploadMovieId {
                title: "duo_la_a_ment",
                rating: 7,
            }
        }
        const MESSAGE: MessageToUploadMovieId = mock_message();
    }
}

#[derive(FaasData, Default, Clone)]
struct MovieTitleId {
    title_id: u64,
    rating: i32,
}

#[no_mangle]
pub fn main() -> Result<()> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "with_libos")] {
            let movie_title =
            DataBuffer::<MessageToUploadMovieId>::from_buffer_slot("upload_movie_title".to_owned())
                .ok_or("missing databuffer, slot: upload_user_name")?;
        } else {
            let movie_title = MESSAGE;
        }
    }

    let title_id = {
        let mut hasher = hashbrown::hash_map::DefaultHashBuilder::default().build_hasher();
        movie_title.title.hash(&mut hasher);
        hasher.finish()
    };

    let mut message = DataBuffer::<MovieTitleId>::with_slot("movie_titie_id".to_owned());
    message.title_id = title_id;
    message.rating = movie_title.rating;

    Ok(().into())
}
