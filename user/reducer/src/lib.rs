#![no_std]
use alloc::{
    borrow::ToOwned,
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
use ms_std::{
    agent::{DataBuffer, FaaSFuncResult as Result, Zero},
    println,
};
use ms_std_proc_macro::Verify;

extern crate alloc;

#[derive(Default, Verify)]
struct Mapper2Reducer {
    shuffle: Vec<BTreeMap<String, u32>>,
}

#[allow(clippy::result_unit_err)]
#[no_mangle]
pub fn main(args: &BTreeMap<String, String>) -> Result<Zero> {
    let my_id = &args["id"];
    let reducer_id: usize = my_id.parse().expect("wrong id.");
    let mapper_num: u64 = args
        .get("mapper_num")
        .expect("missing arg mapper_num")
        .parse()
        .unwrap_or_else(|_| panic!("bad arg, mapper_num={}", args["mapper_num"]));

    let mut counter: BTreeMap<String, u32> = BTreeMap::new();
    for i in 0..mapper_num {
        let mapper_result: DataBuffer<Mapper2Reducer> = DataBuffer::from_buffer_slot(i.to_string())
            .unwrap_or_else(|| panic!("missing mapper result? mapper_id={}", i.to_string()));

        for (word, count) in &mapper_result.shuffle[reducer_id] {
            let old_count = *counter.entry(word.to_owned()).or_insert(0);
            counter.insert(word.to_owned(), old_count + count);
        }
    }

    for (word, count) in counter {
        println!("{}:{}", word, count);
    }

    Ok(Zero::default().into())
}
