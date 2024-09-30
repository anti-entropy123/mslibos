#![no_std]
use alloc::{borrow::ToOwned, format, string::String, vec::Vec};
use hashbrown::HashMap;
pub use ms_hostcall::Verify;
use ms_std::{
    agent::{DataBuffer, FaaSFuncResult as Result},
    args, println,
};
use ms_std_proc_macro::FaasData;

extern crate alloc;

#[derive(Default, FaasData)]
struct Reader2Mapper {
    content: String,
}

#[derive(FaasData)]
struct Mapper2Reducer {
    shuffle: HashMap<String, u32>,
}

impl Default for Mapper2Reducer {
    fn default() -> Self {
        Self {
            shuffle: HashMap::new(),
        }
    }
}

pub fn getidx(word: &str, reducer_num: u64) -> u64 {
    let mut hash_val: u64 = 0;
    for c in word.chars() {
        hash_val = hash_val * 31 + c as u64;
        hash_val = hash_val % reducer_num;
    }
    hash_val
}

#[allow(clippy::result_unit_err)]
#[no_mangle]
pub fn main() -> Result<()> {
    let my_id = args::get("id").unwrap();
    let reducer_num: u64 = args::get("reducer_num")
        .expect("missing arg reducer_num")
        .parse()
        .unwrap_or_else(|_| panic!("bad arg, reducer_num={}", args::get("reducer_num").unwrap()));

    let reader: DataBuffer<Reader2Mapper> =
        DataBuffer::from_buffer_slot(format!("part-{}", my_id)).expect("missing input data.");

    let mut counter = HashMap::new();

    for line in reader.content.lines() {
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
        let mut buffer: DataBuffer<Mapper2Reducer> =
            DataBuffer::with_slot(format!("{}-{}", my_id, reducer));
        buffer.shuffle = Default::default();
        data_buffers.push(buffer);
    }

    ms_std::println!("the counter nums is {}", counter.len());
    for (word, count) in counter {
        let shuffle_idx = getidx(word, reducer_num);

        data_buffers
            .get_mut(shuffle_idx as usize)
            .unwrap_or_else(|| {
                println!("vec get_mut failed, idx={}", shuffle_idx);
                panic!()
            })
            .shuffle
            .insert(word.to_owned(), count);
    }

    Ok(().into())
}
