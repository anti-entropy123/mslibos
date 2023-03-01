mod hostcalls;

use std::path::PathBuf;

pub use hostcalls::find_host_call;
use libloading::{Library, Symbol};

pub type SetHandlerFunc<'a> = Symbol<'a, unsafe extern "C" fn(usize) -> Result<(), ()>>;
pub type GetHandlerFunc<'a> = Symbol<'a, unsafe extern "C" fn() -> usize>;
pub type RustMainFunc<'a> = Symbol<'a, unsafe extern "C" fn() -> ()>;

pub fn load_dynlib(filename: PathBuf) -> anyhow::Result<Library> {
    let lib = unsafe { Library::new(filename) }?;
    anyhow::Ok(lib)
}

pub fn find_symbol<'a, T>(lib: &'a Library, symbol: &str) -> Symbol<'a, T> {
    unsafe { lib.get(symbol.as_bytes()).expect("find symbol failed") }
}
