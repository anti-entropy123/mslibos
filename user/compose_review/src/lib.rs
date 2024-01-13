#![cfg_attr(feature = "with_libos", no_std)]

cfg_if::cfg_if! {
    if #[cfg(feature = "with_libos")] {
        use ms_std::{agent::FaaSFuncResult as Result};
        extern crate alloc;
        use alloc::{collections::BTreeMap, string::{String, ToString}};
    } else {
        type Result<T> = core::result::Result<T, String>;
        use std::collections::BTreeMap;
    }
}

use ms_std::agent::DataBuffer;
use ms_std_proc_macro::FaasData;

#[derive(FaasData, Default, Clone)]
struct MessageToUploadUserId(String);

#[derive(FaasData, Default, Clone)]
struct MessageToUploadMovieId {
    title: String,
}

#[derive(FaasData, Default, Clone)]
struct MessageToMrUploadText {
    text: String,
}

/// Input schema:
/// review = {
///     "username": "abcd",
///     "password": "passwd",
///     "title": "duo_la_a_meng",
///     "rating": 7,
///     "text": "ssxxzzrrbaba"
/// }
///
#[no_mangle]
pub fn main(args: &BTreeMap<String, String>) -> Result<()> {
    let mut message1 =
        DataBuffer::<MessageToUploadUserId>::with_slot("upload_user_name".to_string());
    message1.0 = args
        .get("username")
        .ok_or("missing arg: username".to_string())?
        .clone();

    let mut message2 =
        DataBuffer::<MessageToUploadMovieId>::with_slot("upload_movie_title".to_string());
    message2.title = args
        .get("title")
        .ok_or("missing arg: title".to_string())?
        .clone();

    let mut message3 = DataBuffer::<MessageToMrUploadText>::with_slot("upload_text".to_string());
    message3.text = args
        .get("text")
        .ok_or("missing arg: text".to_string())?
        .clone();

    Ok(().into())
}
