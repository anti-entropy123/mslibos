// provenance is "出处". `strict_provenance` will
// disable casting usize to *const u8.
// #![feature(strict_provenance)]
#![feature(core_panic)]
#![feature(new_uninit)]
#![allow(clippy::result_unit_err)]

mod hostcalls;

pub mod isolation;
pub mod logger;
mod metric;
pub mod service;
pub mod utils;

pub use hostcalls::{GetHandlerFuncSybmol, RustMainFuncSybmol, SetHandlerFuncSybmol};
pub use metric::MetricOpt;
