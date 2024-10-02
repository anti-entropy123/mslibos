#![allow(clippy::result_unit_err)]
#![feature(new_uninit)]

mod hostcalls;

pub mod isolation;
pub mod logger;
mod metric;
pub mod service;
pub mod utils;
use std::sync::Arc;

pub use metric::MetricOpt;

pub use hostcalls::{GetHandlerFuncSybmol, RustMainFuncSybmol, SetHandlerFuncSybmol};

use isolation::{config::IsolationConfig, Isolation};

pub fn run_single_isol<F>(isol_path: String, init_ops: F)
where
    F: FnOnce(),
{
    init_ops();
    let config = IsolationConfig::from_file(isol_path.into()).expect("Open config file failed.");
    let isol = Isolation::new(&config);
    isol.run().expect("isolation user function error.");

    assert_eq!(
        Arc::strong_count(&isol),
        1,
        "isol rc check failed: isol has strong count={}",
        Arc::strong_count(&isol),
    );

    isol.metric.analyze(&MetricOpt::None);
}
