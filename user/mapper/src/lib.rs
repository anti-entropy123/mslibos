#![no_std]
use core::hash::{Hash, Hasher};

use alloc::{borrow::ToOwned, collections::BTreeMap, format, string::String, vec::Vec};
use ms_std::{
    agent::{DataBuffer, FaaSFuncResult as Result},
    fs::File,
    io::Read,
};
use ms_std_proc_macro::Verify;

extern crate alloc;

#[derive(Default, Verify)]
struct Mapper2Reducer {
    shuffle: Vec<BTreeMap<String, u32>>,
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

    let mut data_buffer: DataBuffer<Mapper2Reducer> = DataBuffer::with_slot(my_id.to_owned());
    data_buffer.shuffle = Vec::with_capacity(reducer_num as usize);
    for _ in 0..reducer_num {
        data_buffer.shuffle.push(Default::default())
    }

    for (word, count) in counter {
        let shuffle_idx = {
            let mut hasher = ahash::AHasher::default();
            word.hash(&mut hasher);
            hasher.finish() % reducer_num
        };

        data_buffer
            .shuffle
            .get_mut(shuffle_idx as usize)
            .unwrap_or_else(|| panic!("wrong shuffle num? shuffle_idx={}", shuffle_idx))
            .insert(word.to_owned(), count);
    }

    Ok(().into())
}
