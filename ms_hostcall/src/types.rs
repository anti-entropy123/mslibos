use alloc::string::String;
// use alloc::

use crate::{HostCallID, IsolationContext};

pub type IsolationID = u64;
pub type ServiceName = String;
pub type SymbolName = String;
pub type HostCallResult = Result<(), HostCallError>;

#[derive(Debug)]
pub enum HostCallError {
    HasBeenSet,
}

// msvisor
pub type FindHostCallFunc = unsafe extern "C" fn(IsolationID, HostCallID) -> usize;

// service init
pub type SetHandlerFunc = unsafe extern "C" fn(IsolationContext) -> HostCallResult;
pub type GetHandlerFunc = unsafe extern "C" fn() -> usize;
pub type PanicHandlerFunc = unsafe extern "C" fn() -> !;

// app main
pub type RustMainFunc = unsafe fn() -> ();

// fdtab
pub type HostWriteFunc = fn(i32, &str) -> isize;

// stdio
pub type HostStdioFunc = fn(&str) -> isize;
