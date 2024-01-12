#![cfg_attr(feature = "with_libos", no_std)]

use core::hash::{Hash, Hasher};

cfg_if::cfg_if! {
    if #[cfg(feature = "with_libos")] {
        use core::hash::BuildHasher;
        use ms_std::{agent::{FaaSFuncResult as Result, DataBuffer}};
        use ms_std_proc_macro::FaasData;
        extern crate alloc;
        use alloc::{string::String, borrow::ToOwned};
    } else {
        type Result<T> = core::result::Result<T, String>;
        use std::collections::BTreeMap;
    }
}

#[derive(FaasData, Default, Clone)]
struct MessageToUploadMovieId {
    title: String,
}

#[no_mangle]
pub fn main() -> Result<()> {
    let movie_title =
        DataBuffer::<MessageToUploadMovieId>::from_buffer_slot("upload_movie_title".to_owned())
            .ok_or("missing databuffer, slot: upload_user_name")?;

    let mut hasher = hashbrown::hash_map::DefaultHashBuilder::default().build_hasher();
    movie_title.title.hash(&mut hasher);
    let _title_id = hasher.finish();

    Ok(().into())
}
