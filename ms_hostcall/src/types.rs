use thiserror::Error;

use crate::HostCallID;

pub type FindHostCallFunc = unsafe extern "C" fn(HostCallID) -> usize;
pub type SetHandlerFunc = unsafe extern "C" fn(usize) -> HostCallResult;
pub type GetHandlerFunc = unsafe extern "C" fn() -> usize;
pub type RustMainFunc = unsafe extern "C" fn() -> ();
pub type HostWriteFunc = fn(i32, &str) -> isize;

pub type ServiceName = String;
pub type SymbolName = String;

pub type HostCallResult = Result<(), HostCallError>;
#[derive(Error, Debug)]
pub enum HostCallError {
    #[error("find_hostcall_handler has been set.")]
    HasBeenSet,
}
