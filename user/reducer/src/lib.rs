#![no_std]
use alloc::{
    borrow::ToOwned,
    collections::BTreeMap,
    format,
    string::{String, ToString},
    vec::Vec,
};
pub use ms_hostcall::Verify;
use ms_std::{
    agent::{DataBuffer, FaaSFuncResult as Result},
    println,
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
    let reducer_id: usize = my_id.parse().expect("wrong id.");

    let mapper_num: u64 = args
        .get("mapper_num")
        .expect("missing arg mapper_num")
        .parse()
        .unwrap_or_else(|_| panic!("bad arg, mapper_num={}", args["mapper_num"]));

    let mut counter: BTreeMap<String, u32> = BTreeMap::new();
    for i in 0..mapper_num {
        // println!("need databuffer slot={}-{}", i, reducer_id);
        let mapper_result: DataBuffer<Mapper2Reducer> =
            DataBuffer::from_buffer_slot(format!("{}-{}", i, reducer_id))
                .unwrap_or_else(|| panic!("missing mapper result? mapper_id={}", i.to_string()));

        for (word, count) in &mapper_result.shuffle {
            let old_count = *counter.entry(word.to_owned()).or_insert(0);
            counter.insert(word.to_owned(), old_count + count);
        }
    }

    for (word, count) in counter {
        println!("{}:{}", word, count);
    }

    Ok(().into())
}
