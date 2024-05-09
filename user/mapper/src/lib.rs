#![no_std]
use core::hash::{BuildHasher, Hash, Hasher};

use alloc::{borrow::ToOwned, collections::BTreeMap, format, string::String, vec::Vec};
use hashbrown::HashMap;
pub use ms_hostcall::Verify;
use ms_std::agent::{DataBuffer, FaaSFuncResult as Result};
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

#[allow(clippy::result_unit_err)]
#[no_mangle]
pub fn main(args: &BTreeMap<String, String>) -> Result<()> {
    let my_id = &args["id"];
    let reducer_num: u64 = args
        .get("reducer_num")
        .expect("missing arg reducer_num")
        .parse()
        .unwrap_or_else(|_| panic!("bad arg, reducer_num={}", args["reducer_num"]));

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
        data_buffers.push(DataBuffer::with_slot(format!("{}-{}", my_id, reducer)));
    }

    ms_std::println!("the counter nums is {}", counter.len());
    for (word, count) in counter {
        let shuffle_idx = {
            let mut hasher = hashbrown::hash_map::DefaultHashBuilder::default().build_hasher();
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
