#![no_std]
use core::hash::{Hash, Hasher};

use alloc::{borrow::ToOwned, collections::BTreeMap, format, string::String, vec::Vec};
pub use ms_hostcall::Verify;
use ms_std::{
    agent::{DataBuffer, FaaSFuncResult as Result},
    fs::File,
    io::Read,
};
use ms_std_proc_macro::FaasData;

extern crate alloc;

#[derive(Default, FaasData)]
struct Mapper2Reducer {
    shuffle: BTreeMap<String, u32>,
}

#[allow(clippy::result_unit_err)]
#[no_mangle]
pub fn main(args: &BTreeMap<String, String>) -> Result<()> {
    let my_id = &args["id"];
    let reducer_num: u64 = args
        .get("reducer_num")
        .expect("missing arg reducer_num")
        .parse()
        .unwrap_or_else(|_| panic!("bad arg, reducer_num={}", args["reducer_num"]));

    let content = {
        let mut f = File::open(&format!("part-{}", my_id))?;
        let mut buf = String::new();
        f.read_to_string(&mut buf).expect("read file failed.");
        buf
    };

    let mut counter = BTreeMap::new();

    for line in content.lines() {
        let words = line
            .trim()
            .split(' ')
            .filter(|word| word.chars().all(char::is_alphanumeric));

        for word in words {
            let old_count = *counter.entry(word).or_insert(0u32);
            counter.insert(word, old_count + 1);
        }
    }

    let mut data_buffers: Vec<DataBuffer<Mapper2Reducer>> =
        Vec::with_capacity(reducer_num as usize);

    for reducer in 0..reducer_num {
        data_buffers.push(DataBuffer::with_slot(format!("{}-{}", my_id, reducer)));
    }

    for (word, count) in counter {
        let shuffle_idx = {
            let mut hasher = ahash::AHasher::default();
            word.hash(&mut hasher);
            hasher.finish() % reducer_num
        };

        data_buffers
            .get_mut(shuffle_idx as usize)
            .unwrap()
            .shuffle
            .insert(word.to_owned(), count);
    }

    Ok(().into())
}
