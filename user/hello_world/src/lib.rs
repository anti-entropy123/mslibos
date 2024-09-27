#![cfg_attr(feature = "with_libos", no_std)]

use alloc::borrow::ToOwned;
use ms_std::args;

cfg_if::cfg_if! {
    if #[cfg(feature = "with_libos")] {
        use ms_std::{agent::FaaSFuncResult as Result, println};
        extern crate alloc;
        use alloc::{collections::BTreeMap, string::String};
    } else {
        type Result<T> = core::result::Result<T, String>;
        use std::collections::BTreeMap;
    }
}

static TEXT: &str = include_str!("../fake_data_0.txt");

#[no_mangle]
pub fn main() -> Result<()> {
    // let id = args::get("id").unwrap();
    // println!("Hello, world! id: {}", id);

    let mut hasher: hashbrown::HashMap<String, u32> = hashbrown::HashMap::new();

    for word in TEXT.split_whitespace() {
        // println!("{}", word)
        hasher.insert(word.to_owned(), hasher.get(word).unwrap_or(&0) + 1);
    }
    println!("hash insert ok");

    #[cfg(feature = "measure_mem")]
    {
        use ms_std::libos::MetricEvent::Mem;
        ms_std::libos::metric(Mem);
    }

    Ok(().into())
}
