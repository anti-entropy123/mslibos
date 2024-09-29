#![cfg_attr(feature = "with_libos", no_std)]

cfg_if::cfg_if! {
    if #[cfg(feature = "with_libos")] {
        use ms_std::{agent::FaaSFuncResult as Result};
        extern crate alloc;
        use alloc::{collections::BTreeMap, string::String};
    } else {
        type Result<T> = core::result::Result<T, String>;
        use std::collections::BTreeMap;
    }
}

use alloc::{ format, string::ToString, vec::Vec};
use ms_std::{args, println, libos::libos};
use ms_hostcall::types::{OpenFlags, OpenMode};

use tinywasm::{Module, Store, ModuleInstance};
use wasi_api::tinywasm;

const WASM: &[u8] = include_bytes!("../reducer.wasm");

lazy_static::lazy_static! {
    static ref MUST_OPEN_ROOT: bool = {
        libos!(open("/", OpenFlags::empty(), OpenMode::RD)).unwrap();
        true
    };
}

#[no_mangle]
pub fn main() -> Result<()> {
    let my_id = args::get("id").unwrap();
    let mapper_num: u64 = args::get("mapper_num")
        .expect("missing arg mapper_num")
        .parse()
        .unwrap_or_else(|_| panic!("bad arg, mapper_num={}", args::get("mapper_num").unwrap()));
    println!("rust: my_id: {:?}, mapper_num: {:?}", my_id, mapper_num);
    let mut wasi_args: Vec<String> = Vec::new();
    wasi_args.push("fake system path!".to_string()); // c语言main第一个参数是系统路径
    wasi_args.push(my_id.to_string());
    wasi_args.push(mapper_num.to_string());

    let _open_root = *MUST_OPEN_ROOT;

    let module = Module::parse_bytes(WASM)?;
    let mut store = Store::default();
    let imports = wasi_api::import_all()?;

    wasi_api::set_wasi_args(store.id(), wasi_args);

    let instance = ModuleInstance::instantiate(&mut store, module, Some(imports))?;
    let main = instance.exported_func::<(), ()>(&store, "_start")?;

    if let Err(e) = unwinding::panic::catch_unwind(|| main.call(&mut store, ()).unwrap()) {
        let msg = format!("{:?}", e);
        println!("err msg: {}", msg);
        if msg != "normally exit" {
            // return Err();
        }
    };

    Ok(().into())
}
