// provenance is "出处". `strict_provenance` will
// disable casting usize to *const u8.
// #![feature(strict_provenance)]
#![feature(core_panic)]
mod hostcalls;

pub mod isolation;
pub mod logger;
mod metric;
pub mod service;
pub mod utils;

pub use hostcalls::{GetHandlerFuncSybmol, RustMainFuncSybmol, SetHandlerFuncSybmol};
