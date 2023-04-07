// provenance is "出处". `strict_provenance` will 
// disable casting usize to *const u8. 
// #![feature(strict_provenance)]

mod hostcalls;

pub mod isolation;
pub mod logger;
pub mod service;
pub mod utils;
mod metric;

pub use hostcalls::{GetHandlerFuncSybmol, RustMainFuncSybmol, SetHandlerFuncSybmol};
