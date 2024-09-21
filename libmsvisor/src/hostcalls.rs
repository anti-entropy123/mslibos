use libloading::Symbol;
use ms_hostcall::types::{GetHandlerFunc, RustMainFunc, SetHandlerFunc};

pub type SetHandlerFuncSybmol<'a> = Symbol<'a, SetHandlerFunc>;
pub type GetHandlerFuncSybmol<'a> = Symbol<'a, GetHandlerFunc>;
pub type RustMainFuncSybmol<'a> = Symbol<'a, RustMainFunc>;
