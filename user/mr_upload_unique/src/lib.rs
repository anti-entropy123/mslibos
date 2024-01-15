#![cfg_attr(feature = "with_libos", no_std)]

use rand::{rngs::SmallRng, Rng, SeedableRng};

cfg_if::cfg_if! {
    if #[cfg(feature = "with_libos")] {
        use ms_std::{agent::FaaSFuncResult as Result, time::{SystemTime, UNIX_EPOCH}};
        extern crate alloc;
        use alloc::{format, string::{String, ToString}, borrow::ToOwned};
    } else {
        type Result<T> = core::result::Result<T, String>;
        use std::time::{SystemTime, UNIX_EPOCH};
    }
}

use ms_std::agent::DataBuffer;
use ms_std_proc_macro::FaasData;

#[derive(FaasData, Default, Clone)]
struct UniqueReviewId(String);

fn generate_random_number(rng: &mut SmallRng, digits: usize) -> u64 {
    // 计算上下界限
    let lower_bound = 10u64.pow(digits as u32 - 1);
    let upper_bound = 10u64.pow(digits as u32) - 1;

    // 生成随机数
    rng.gen_range(lower_bound..=upper_bound)
}

#[no_mangle]
pub fn main() -> Result<()> {
    let mut rng = rand::rngs::SmallRng::from_seed(Default::default());
    let machine_id = generate_random_number(&mut rng, 2);
    #[cfg(feature = "with_libos")]
    let timestamp =
        (SystemTime::now().duration_since(UNIX_EPOCH).as_millis() - 1514764800000).to_string();
    #[cfg(not(feature = "with_libos"))]
    let timestamp = (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "get timestamp failed")?
        .as_millis()
        - 1514764800000)
        .to_string();

    let timestamp = &timestamp[(timestamp.len() - 11)..];
    let index_id = generate_random_number(&mut rng, 3);
    let review_id = format!("{}{}{}", machine_id, timestamp, index_id,);

    let mut message = DataBuffer::<UniqueReviewId>::with_slot("unique_review_id".to_owned());
    message.0 = review_id;

    Ok(().into())
}
